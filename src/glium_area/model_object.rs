use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use glium::{DrawParameters, Frame, IndexBuffer, Surface, uniform, VertexBuffer};
use glium::backend::Context;
use glium::index::PrimitiveType;
use image::Rgba;
use nalgebra_glm as glm;
use nalgebra_glm::Mat4;

use crate::glium_area::camera::Camera;
use crate::glium_area::cross_info::CrossInfo;
use crate::glium_area::cube_side::CubeSide;
use crate::glium_area::model_object::ModelIndexType::TrianglesList;
use crate::glium_area::ray::Ray;
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
}

pub enum ModelIndexType {
    TrianglesList(Vec<u16>),
    LinesList
}

impl ModelObject {
    pub fn new(
        context: Rc<Context>,
        program: Rc<glium::Program>,
        camera: Rc<RefCell<Camera>>,
        vertexes: &[Vertex],
        indexes: ModelIndexType,
        translation_vector: &glm::Vec3,
        scale_vector: &glm::Vec3
    ) -> Self
    {
        let model_matrix = glm::Mat4::identity();
        let translation_matrix = glm::translate(&glm::Mat4::identity(), translation_vector);
        let scale_matrix = glm::scale(&glm::Mat4::identity(),scale_vector);

        let vertexes = vertexes.to_vec();
        let vertex_buffer = VertexBuffer::new(&context, &vertexes).unwrap();

        let mut draw_parameters: Option<DrawParameters> = None;
        let mut index_buffer: Option<IndexBuffer<u16>> = None;
        match indexes {
            TrianglesList(data) => {
                index_buffer = Some(IndexBuffer::new(&context, PrimitiveType::TrianglesList, &data).unwrap());
                draw_parameters = Some(DrawParameters {
                    blend: glium::Blend::alpha_blending(),
                    backface_culling: glium::BackfaceCullingMode::CullCounterClockwise,
                    ..Default::default()
                });
            },
            ModelIndexType::LinesList => {
                let mut data = Vec::with_capacity(vertexes.len());
                for i in 0..vertexes.len() {
                    data.push(i as u16);
                }
                index_buffer = Some(IndexBuffer::new(&context, PrimitiveType::LinesList, &data).unwrap());
                draw_parameters = Some(DrawParameters {
                    // line_width: Some(5.0),
                    ..Default::default()
                });
            }
        };

        ModelObject {
            context, program, camera,
            model_matrix,
            draw_parameters: draw_parameters.unwrap(),
            vertexes,
            vertex_buffer,
            index_buffer: index_buffer.unwrap(),
            translation_matrix,
            scale_matrix,
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

        let uniforms = uniform!{
            model_matrix: *self.model_matrix.as_ref(),
            view_matrix: *view_matrix.as_ref(),
            perspective_matrix: *projection_matrix.as_ref(),
        };
        frame.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniforms,
            &self.draw_parameters,
        ).unwrap();
    }

    pub fn paint(&mut self, cell: usize, color: [f32; 4]) {
        let cell = cell + 1;
        let chunk_size = 4;
        let index = cell * chunk_size - 1;
        self.vertexes.get_mut(index - 0).unwrap().color = color;
        self.vertexes.get_mut(index - 1).unwrap().color = color;
        self.vertexes.get_mut(index - 2).unwrap().color = color;
        self.vertexes.get_mut(index - 3).unwrap().color = color;

        self.vertex_buffer = VertexBuffer::new(&self.context, &self.vertexes).unwrap();
    }

    pub fn set_pixels(&mut self, color_map: &BTreeMap<CubeSide, Vec<Rgba<u8>>>) {
        let mut cell = 0;
        for pixels in color_map.values() {
            for pixel in pixels {
                let pixel_f32 = ModelObject::u8_to_f32_pixel(pixel);
                self.paint(cell, pixel_f32);
                cell += 1;
            }
        }

        // TEST specific SIDE
        // let sides = [CubeSide::Front, CubeSide::Left, CubeSide::Back, CubeSide::Right, CubeSide::Top, CubeSide::Bottom];
        // for side in sides {
        //     for pixel in color_map.get(&side).unwrap() {
        //         let pixel_f32 = ModelObject::u8_to_f32_pixel(pixel);
        //         self.paint(cell, pixel_f32);
        //         cell += 1;
        //     }
        // }

    }

    pub fn get_pixel(&self, cell: usize) -> [f32; 4] {
        self.vertexes.get(4 * cell).unwrap().color
    }
    
    fn u8_to_f32_pixel(pixel: &Rgba<u8>) -> [f32; 4] {
        [
            f32::from(pixel[0]) / 255.0,
            f32::from(pixel[1]) / 255.0,
            f32::from(pixel[2]) / 255.0,
            f32::from(pixel[3]) / 255.0,
        ]
    }

    fn u8_color_to_f32_color(u8_color: u8) -> f32 {
        let f32_color = f32::from(u8_color) / 255.0;
        return f32_color;
    }

    fn get_projection(&self) -> Mat4 {
        let (width, height) = self.context.get_framebuffer_dimensions();
        let aspect_ratio = width as f32 / height as f32 ;
        let fov: f32 = std::f32::consts::PI / 3.0; // 60 degrees
        let near = 0.1;
        let far = 1000.0;
        glm::perspective_rh(aspect_ratio, fov, near, far)
    }

    pub fn cross(&self, ray: &Ray) -> Option<CrossInfo> {
        let cells: Vec<[Vertex; 4]> = self.vertexes
            .chunks(4)
            .map(|chunk| {
                [chunk[0], chunk[1], chunk[2], chunk[3]]
            })
            .collect();

        let mut result: Vec<CrossInfo> = vec![];
        for (cell_index, cell) in cells.iter().enumerate() {
            let res = self.cross_with_cell(ray, cell);
            if let Some((distance, coords)) = res {
                result.push(CrossInfo { cell_index: cell_index, dist: distance });
            }
        }

        result
            .iter()
            .min_by(|cross1, cross2| {
                cross1.dist.total_cmp(&cross2.dist)
            })
            .cloned()
    }

    fn cross_with_cell(&self, ray: &Ray, face: &[Vertex; 4]) -> Option<(f32, glm::Vec3)> {
        let transformed_face: [glm::Vec4; 4] = face.iter()
            .map(|vertex| {
                let position = glm::Vec4::new(
                    vertex.position[0],
                    vertex.position[1],
                    vertex.position[2],
                    1.0);
                glm::Vec4::from(self.model_matrix * position)
            })
            .collect::<Vec<glm::Vec4>>()
            .try_into()
            .unwrap();

        let triangle1 = [transformed_face[0].xyz(), transformed_face[1].xyz(), transformed_face[2].xyz()];
        let triangle2 = [transformed_face[0].xyz(), transformed_face[3].xyz(), transformed_face[2].xyz()];

        let intersection1 = self.cross_with_triangle(&ray, triangle1);
        if intersection1.is_some() { return intersection1; }

        let intersection2 = self.cross_with_triangle(&ray, triangle2);
        if intersection2.is_some() { return intersection2; }

        None
    }

    fn cross_with_triangle(&self, ray: &Ray, triangle: [glm::Vec3; 3]) -> Option<(f32, glm::Vec3)> {
        // Moller-Trumbore algorithm
        let edge1 = triangle[1] - triangle[0];
        let edge2 = triangle[2] - triangle[0];
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);

        // Проверка на параллельность луча и треугольника
        if a.abs() < 0.000001 {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - triangle[0];
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // Параметр t для точки пересечения
        let t = f * edge2.dot(&q);

        // Проверяем, что пересечение находится в пределах луча
        if t > 0.0 {
            let cross_point = glm::Vec3::new(
                ray.origin.x + t * ray.direction.x,
                ray.origin.y + t * ray.direction.y,
                ray.origin.z + t * ray.direction.z
            );
            Some((t, cross_point))
        } else {
            None
        }
    }
}