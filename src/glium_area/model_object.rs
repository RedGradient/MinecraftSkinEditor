use std::cell::RefCell;
use std::rc::Rc;

use glium::{DrawParameters, Frame, IndexBuffer, Surface, uniform, VertexBuffer};
use glium::backend::Context;
use glium::index::PrimitiveType;
use image::Rgba;
use nalgebra_glm as glm;
use nalgebra_glm::Mat4;

use crate::glium_area::camera::Camera;
use crate::glium_area::cross_info::CrossInfo;
use crate::glium_area::model::generate_indexes;
use crate::glium_area::pick::{cell_range_for_face, local_hit_distance_on_ray, ray_local_aabb, world_ray_to_local};
use crate::glium_area::ray::Ray;
use crate::glium_area::skin_parser::CubeSideColors;
use crate::glium_area::vertex::Vertex;

pub struct ModelObject {
    context: Rc<Context>,
    program: Rc<glium::Program>,
    camera: Rc<RefCell<Camera>>,

    draw_parameters: DrawParameters<'static>,

    vertexes: Vec<Vertex>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,

    model_matrix: Mat4,
    translation_matrix: Mat4,
    scale_matrix: Mat4,

    local_bounds_min: glm::Vec3,
    local_bounds_max: glm::Vec3,
    cells_per_side: [usize; 6],
}

#[derive(Clone, Copy)]
pub enum ModelObjectType {
    Model,
    Grid
}

impl ModelObject {
    pub fn new(
        context: Rc<Context>,
        program: Rc<glium::Program>,
        camera: Rc<RefCell<Camera>>,
        vertexes: &[Vertex],
        model_object_type: ModelObjectType,
        translation_vector: &glm::Vec3,
        scale_vector: &glm::Vec3,
        cells_per_side: [usize; 6],
    ) -> Self {
        let model_matrix = glm::Mat4::identity();
        let translation_matrix = glm::translate(&glm::Mat4::identity(), translation_vector);
        let scale_matrix = glm::scale(&glm::Mat4::identity(), scale_vector);
        let index_buffer = Self::create_index_buffer(context.clone(), vertexes, model_object_type);
        let draw_parameters = Self::create_draw_parameters(model_object_type);
        let vertexes = vertexes.to_vec();
        let vertex_buffer = VertexBuffer::dynamic(&context, &vertexes).expect("Cannot create vertex buffer");
        let (local_bounds_min, local_bounds_max) = Self::compute_bounds(&vertexes);

        ModelObject {
            context,
            program,
            camera,
            model_matrix,
            draw_parameters,
            vertexes,
            vertex_buffer,
            index_buffer,
            translation_matrix,
            scale_matrix,
            local_bounds_min,
            local_bounds_max,
            cells_per_side,
        }
    }

    fn compute_bounds(vertexes: &[Vertex]) -> (glm::Vec3, glm::Vec3) {
        let mut min = glm::vec3(f32::MAX, f32::MAX, f32::MAX);
        let mut max = glm::vec3(f32::MIN, f32::MIN, f32::MIN);
        for vertex in vertexes {
            for axis in 0..3 {
                min[axis] = min[axis].min(vertex.position[axis]);
                max[axis] = max[axis].max(vertex.position[axis]);
            }
        }
        (min, max)
    }

    fn create_index_buffer(context: Rc<Context>, vertexes: &[Vertex], model_object_type: ModelObjectType) -> IndexBuffer<u16> {
        match model_object_type {
            ModelObjectType::Model => {
                let data = generate_indexes(vertexes.len() / 4);
                IndexBuffer::new(&context, PrimitiveType::TrianglesList, &data).expect("Cannot create index buffer")
            }
            ModelObjectType::Grid => {
                let data: Vec<u16> = (0..vertexes.len() as u16).collect();
                IndexBuffer::new(&context, PrimitiveType::LinesList, &data).unwrap()
            }
        }
    }

    fn create_draw_parameters(model_object_type: ModelObjectType) -> DrawParameters<'static> {
        match model_object_type {
            ModelObjectType::Model => DrawParameters {
                blend: glium::Blend::alpha_blending(),
                backface_culling: glium::BackfaceCullingMode::CullClockwise,
                ..Default::default()
            },
            ModelObjectType::Grid => DrawParameters {
                ..Default::default()
            },
        }
    }

    pub fn get_vertexes(&self) -> Vec<Vertex> {
        self.vertexes.clone()
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let rotation_matrix = self.camera.borrow().get_rotation_matrix();
        self.model_matrix = rotation_matrix * self.translation_matrix * self.scale_matrix;
        let view_matrix = self.camera.borrow().get_view_matrix();
        let projection_matrix = self.get_projection();

        let uniforms = uniform! {
            model_matrix: *self.model_matrix.as_ref(),
            view_matrix: *view_matrix.as_ref(),
            perspective_matrix: *projection_matrix.as_ref(),
        };
        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniforms,
                &self.draw_parameters,
            )
            .unwrap();
    }

    pub fn paint(&mut self, cell: usize, color: [f32; 4]) {
        let index = cell * 4;
        for offset in 0..4 {
            self.vertexes[index + offset].color = color;
        }
        self.write_cell_vertices(index);
    }

    pub fn clear(&mut self) {
        for vertex in self.vertexes.iter_mut() {
            vertex.color = [0.0, 0.0, 0.0, 0.0];
        }
        self.vertex_buffer.write(&self.vertexes);
    }

    fn write_cell_vertices(&self, index: usize) {
        self.vertex_buffer
            .slice(index..index + 4)
            .unwrap()
            .write(&self.vertexes[index..index + 4]);
    }

    pub fn set_pixels(&mut self, color_map: &CubeSideColors, ignore_transparent: bool) {
        let mut cell = 0;
        for pixels in color_map.values() {
            for pixel in pixels {
                let pixel_f32 = ModelObject::u8_to_f32_pixel(pixel);
                if ignore_transparent && *pixel_f32.last().unwrap() == 0.0 {
                    cell += 1;
                    continue;
                }
                self.paint(cell, pixel_f32);
                cell += 1;
            }
        }
    }

    pub fn get_pixel(&self, cell: usize) -> [f32; 4] {
        self.vertexes[4 * cell].color
    }

    pub fn get_pixels(&self) -> Vec<[f32; 4]> {
        self.vertexes
            .chunks(4)
            .map(|chunk| chunk[0].color)
            .collect()
    }

    fn u8_to_f32_pixel(pixel: &Rgba<u8>) -> [f32; 4] {
        [
            f32::from(pixel[0]) / 255.0,
            f32::from(pixel[1]) / 255.0,
            f32::from(pixel[2]) / 255.0,
            f32::from(pixel[3]) / 255.0,
        ]
    }

    fn get_projection(&self) -> Mat4 {
        let (width, height) = self.context.get_framebuffer_dimensions();
        let aspect_ratio = width as f32 / height as f32;
        let fov: f32 = std::f32::consts::PI / 3.0;
        let near = 0.1;
        let far = 1000.0;
        glm::perspective_rh(aspect_ratio, fov, near, far)
    }

    fn object_world_matrix(&self) -> Mat4 {
        let rotation_matrix = self.camera.borrow().get_rotation_matrix();
        rotation_matrix * self.translation_matrix * self.scale_matrix
    }

    pub fn cross(&self, ray: &Ray) -> Option<CrossInfo> {
        let object_matrix = self.object_world_matrix();
        let (local_origin, local_direction) = world_ray_to_local(ray, &object_matrix)?;
        let (t_local, face) = ray_local_aabb(
            local_origin,
            local_direction,
            self.local_bounds_min,
            self.local_bounds_max,
        )?;

        let cell_range = cell_range_for_face(&self.cells_per_side, face);
        let mut closest_intersection: Option<CrossInfo> = None;

        for cell_index in cell_range {
            let vertex_offset = cell_index * 4;
            let face_vertices = [
                self.vertexes[vertex_offset],
                self.vertexes[vertex_offset + 1],
                self.vertexes[vertex_offset + 2],
                self.vertexes[vertex_offset + 3],
            ];

            if let Some(local_t) =
                self.cross_with_cell_local(local_origin, local_direction, &face_vertices, t_local)
            {
                let dist = local_hit_distance_on_ray(
                    &object_matrix,
                    local_origin,
                    local_direction,
                    local_t,
                    ray,
                );
                if closest_intersection
                    .as_ref()
                    .is_none_or(|closest| dist < closest.dist)
                {
                    closest_intersection = Some(CrossInfo { cell_index, dist });
                }
            }
        }

        closest_intersection
    }

    fn cross_with_cell_local(
        &self,
        origin: glm::Vec3,
        direction: glm::Vec3,
        face: &[Vertex; 4],
        min_t: f32,
    ) -> Option<f32> {
        let positions = [
            glm::vec3(face[0].position[0], face[0].position[1], face[0].position[2]),
            glm::vec3(face[1].position[0], face[1].position[1], face[1].position[2]),
            glm::vec3(face[2].position[0], face[2].position[1], face[2].position[2]),
            glm::vec3(face[3].position[0], face[3].position[1], face[3].position[2]),
        ];

        let triangle1 = [positions[0], positions[1], positions[2]];
        let triangle2 = [positions[0], positions[3], positions[2]];

        let mut closest = None;
        for triangle in [triangle1, triangle2] {
            if let Some(dist) = self.cross_with_triangle_local(origin, direction, triangle) {
                if dist + EPSILON < min_t {
                    continue;
                }
                if closest.is_none_or(|best| dist < best) {
                    closest = Some(dist);
                }
            }
        }
        closest
    }

    fn cross_with_triangle_local(
        &self,
        origin: glm::Vec3,
        direction: glm::Vec3,
        triangle: [glm::Vec3; 3],
    ) -> Option<f32> {
        let edge1 = triangle[1] - triangle[0];
        let edge2 = triangle[2] - triangle[0];
        let normal = edge1.cross(&edge2);
        if normal.dot(&direction) >= 0.0 {
            return None;
        }

        let h = direction.cross(&edge2);
        let a = edge1.dot(&h);
        if a.abs() < 0.000001 {
            return None;
        }

        let f = 1.0 / a;
        let s = origin - triangle[0];
        let u = f * s.dot(&h);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * direction.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);
        if t > 0.0 { Some(t) } else { None }
    }
}

const EPSILON: f32 = 1e-4;
