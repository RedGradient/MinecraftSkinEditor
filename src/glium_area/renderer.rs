use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::rc::Rc;

use glium::{Frame, Surface};
use glium::backend::Context;
use image::{ImageBuffer, Rgba};
use nalgebra_glm as glm;
use nalgebra_glm::Mat4;

use crate::glium_area::body_part::BodyPart;
use crate::glium_area::body_part::BodyPart::*;
use crate::glium_area::camera::Camera;
use crate::glium_area::cross_info::CrossInfo;
use crate::glium_area::hover_state::HoverState;
use crate::glium_area::model::{arm_fn, body_fn, head_fn};
use crate::glium_area::model_object::ModelObject;
use crate::glium_area::mouse_move::MouseMove;
use crate::glium_area::ray::Ray;
use crate::glium_area::skin_parser::SkinParser;
use crate::glium_area::vertex::Vertex;

pub struct ModelCell {
    pub body_part: BodyPart,
    pub cell_index: usize,
    pub color: [f32; 4],
}


pub struct Renderer {
    pub context: Rc<Context>,
    program: Rc<glium::Program>,
    camera: Rc<RefCell<Camera>>,
    mouse_motion: Option<MouseMove>,
    mouse_move_on_model: bool,
    projection_matrix: Mat4,
    transform_matrix: Mat4,
    model_objects: BTreeMap<BodyPart, ModelObject>,
    visible_objects: BTreeSet<BodyPart>,
    current_color: glm::Vec4,
    mouse_hover: Option<HoverState>,
}

impl Renderer {
    pub fn new(context: Rc<glium::backend::Context>) -> Self {
        let vertex_shader = Renderer::get_shader_source("resources/shaders/vertex.glsl");
        let fragment_shader = Renderer::get_shader_source("resources/shaders/fragment.glsl");

        let program = glium::Program::from_source(
            &context,
            vertex_shader.as_str(),
            fragment_shader.as_str(),
            None,
        ).unwrap();
        let program = Rc::new(program);
        let camera = Rc::new(RefCell::new(Camera::new()));
        let projection_matrix = glm::Mat4::identity();

        let head = ModelObject::new(
            context.clone(),
            program.clone(),
            camera.clone(),
            &head_fn::head_vertexes(),
            &head_fn::head_indexes(),
            &[],
            &[],
            &glm::Vec3::new(0., 1.5, 0.),
            &glm::Vec3::new(1., 1., 1.),
        );

        let body = ModelObject::new(
            context.clone(),
            program.clone(),
            camera.clone(),
            &body_fn::body_vertexes(),
            &body_fn::body_indexes(),
            &[],
            &[],
            &glm::Vec3::new(0., 0.25, 0.),
            &glm::Vec3::new(1., 1., 1.),
        );

        let right_arm = ModelObject::new(
            context.clone(),
            program.clone(),
            camera.clone(),
            &arm_fn::arm_vertexes(),
            &arm_fn::arm_indexes(),
            // &Renderer::arm_front_lines(),
            &[],
            &[],
            &glm::Vec3::new(-0.75, 0.25, 0.),
            &glm::Vec3::new(1., 1., 1.),
        );

        let left_arm = ModelObject::new(
            context.clone(),
            program.clone(),
            camera.clone(),
            &arm_fn::arm_vertexes(),
            &arm_fn::arm_indexes(),
            &[],
            &[],
            &glm::Vec3::new(0.75, 0.25, 0.),
            &glm::Vec3::new(1., 1., 1.),
        );

        let right_leg = ModelObject::new(
            context.clone(),
            program.clone(),
            camera.clone(),
            &arm_fn::arm_vertexes(),
            &arm_fn::arm_indexes(),
            &[],
            &[],
            &glm::Vec3::new(-0.25, -1.25, 0.),
            &glm::Vec3::new(1., 1., 1.),
        );

        let left_leg = ModelObject::new(
            context.clone(),
            program.clone(),
            camera.clone(),
            &arm_fn::arm_vertexes(),
            &arm_fn::arm_indexes(),
            &[],
            &[],
            &glm::Vec3::new(0.25, -1.25, 0.),
            &glm::Vec3::new(1., 1., 1.),
        );

        const OUTER_SCALE_VECTOR: glm::Vec3 = glm::Vec3::new(1.15, 1.15, 1.15);

        let head_outer = ModelObject::new(
            context.clone(),
            program.clone(),
            camera.clone(),
            &head_fn::head_vertexes(),
            &head_fn::head_indexes(),
            &[],
            &[],
            &glm::Vec3::new(0.0, 1.5, 0.0),
            &OUTER_SCALE_VECTOR,
        );

        let body_outer = ModelObject::new(
            context.clone(),
            program.clone(),
            camera.clone(),
            &body_fn::body_vertexes(),
            &body_fn::body_indexes(),
            &[],
            &[],
            &glm::Vec3::new(0., 0.25, 0.),
            &OUTER_SCALE_VECTOR,
        );

        let right_arm_outer = ModelObject::new(
            context.clone(),
            program.clone(),
            camera.clone(),
            &arm_fn::arm_vertexes(),
            &arm_fn::arm_indexes(),
            &[],
            &[],
            &glm::Vec3::new(-0.75, 0.25, 0.),
            &OUTER_SCALE_VECTOR,
        );

        let left_arm_outer = ModelObject::new(
            context.clone(),
            program.clone(),
            camera.clone(),
            &arm_fn::arm_vertexes(),
            &arm_fn::arm_indexes(),
            &[],
            &[],
            &glm::Vec3::new(0.75, 0.25, 0.),
            &OUTER_SCALE_VECTOR,
        );

        let right_leg_outer = ModelObject::new(
            context.clone(),
            program.clone(),
            camera.clone(),
            &arm_fn::arm_vertexes(),
            &arm_fn::arm_indexes(),
            &[],
            &[],
            &glm::Vec3::new(-0.25, -1.25, 0.),
            &OUTER_SCALE_VECTOR,
        );

        let left_leg_outer = ModelObject::new(
            context.clone(),
            program.clone(),
            camera.clone(),
            &arm_fn::arm_vertexes(),
            &arm_fn::arm_indexes(),
            &[],
            &[],
            &glm::Vec3::new(0.25, -1.25, 0.),
            &OUTER_SCALE_VECTOR,
        );

        let mut model_objects: BTreeMap<BodyPart, ModelObject> = BTreeMap::new();
        // --- INNER LAYER ---
        model_objects.insert(BodyPart::Head, head);
        model_objects.insert(BodyPart::Torso, body);
        model_objects.insert(BodyPart::RightArm, right_arm);
        model_objects.insert(BodyPart::LeftArm, left_arm);
        model_objects.insert(BodyPart::RightLeg, right_leg);
        model_objects.insert(BodyPart::LeftLeg, left_leg);
        // --- OUTER LAYER ---
        model_objects.insert(BodyPart::HeadOuter, head_outer);
        model_objects.insert(BodyPart::TorsoOuter, body_outer);
        model_objects.insert(BodyPart::RightArmOuter, right_arm_outer);
        model_objects.insert(BodyPart::LeftArmOuter, left_arm_outer);
        model_objects.insert(BodyPart::RightLegOuter, right_leg_outer);
        model_objects.insert(BodyPart::LeftLegOuter, left_leg_outer);

        let mut visible_objects = BTreeSet::from([
            Head, Torso, RightArm, LeftArm, RightLeg, LeftLeg,
            HeadOuter, TorsoOuter, RightArmOuter, LeftArmOuter, RightLegOuter, LeftLegOuter
        ]);

        Renderer {
            context,
            program,
            camera,
            mouse_motion: None,
            mouse_move_on_model: false,
            projection_matrix,
            transform_matrix: glm::Mat4::identity(),
            model_objects,
            visible_objects,
            current_color: glm::Vec4::new(0., 0., 1., 1.),
            mouse_hover: None,
        }
    }

    fn arm_front_lines() -> Vec<Vertex> {
        let mut lines = Vec::new();

        let width: u32 = 4;
        let height: u32 = 12;

        let grid_size = 8.0;
        let cell_size = 1.0 / grid_size;

        let z = 0.25;

        for i in 0..height {
            let y = 0.75 - i as f32 * cell_size;
            for j in 0..width {
                let x = -0.25 + j as f32 * cell_size;

                // --- 2 VERTICAL LINES ---
                lines.push(Vertex { position: [x, -0.75, z], color: [0., 0., 0., 1.0]});
                lines.push(Vertex { position: [x, 0.75, z], color: [0., 0., 0., 1.0]});
            }

            // --- 2 HORIZONTAL LINES ---
            lines.push(Vertex { position: [-0.25, y, z], color: [0., 0., 0., 1.0]});
            lines.push(Vertex { position: [0.25, y, z], color: [0., 0., 0., 1.0]});
        }

        lines
    }

    pub fn load_texture(&mut self, path: &str) {
        let parser = SkinParser::new();
        let color_map = match parser.load(path) {
            Ok(color_map) => {
                println!("Texture loaded");
                color_map
            },
            Err(value) => panic!("{}", value)
        };

        for (body_part, cell_object) in self.model_objects.iter_mut() {
            match color_map.get(body_part) {
                Some(color_map) => cell_object.set_pixels(color_map),
                None => ()
            }
        }
    }

    fn get_shader_source(filename: &str) -> String {
        let mut file = File::open(filename).expect("File not found");
        let mut shader_src = String::new();
        file.read_to_string(&mut shader_src).expect("Unable to read the file");
        shader_src
    }

    pub fn get_mouse_hover(&self) -> Option<HoverState> {
        self.mouse_hover
    }

    pub fn set_color(&mut self, color: &glm::Vec4) {
        self.current_color = *color;
    }

    pub fn draw(&mut self) {
        self.projection_matrix = {
            let (width, height) = self.context.get_framebuffer_dimensions();
            let aspect_ratio = width as f32 / height as f32;
            let fov: f32 = std::f32::consts::PI / 3.0; // 60 degrees
            let near = 0.1;
            let far = 1000.0;
            glm::perspective_rh(aspect_ratio, fov, near, far)
        };

        let mut frame = Frame::new(
            self.context.clone(),
            self.context.get_framebuffer_dimensions(),
        );

        frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

        for body_part in &self.visible_objects {
            self.model_objects.get_mut(body_part).unwrap().draw(&mut frame);
        }

        frame.finish().unwrap();
    }

    pub fn mouse_move(&mut self, curr_x: f32, curr_y: f32) {
        if self.mouse_motion.is_some() {
            let mut mm = self.mouse_motion.take().unwrap();
            mm.move_to(curr_x, curr_y);
            self.mouse_motion = Some(mm);
        }
    }

    pub fn is_motion_on_empty_area(&self) -> bool {
        self.mouse_motion.is_some()
    }

    pub fn start_motion(&mut self, curr_x: f32, curr_y: f32) { self.mouse_motion = Some(MouseMove::new(curr_x, curr_y)) }
    pub fn stop_motion(&mut self) { self.mouse_motion = None; }

    pub fn set_mouse_hover(&mut self, hover: Option<HoverState>) { self.mouse_hover = hover; }

    pub fn update_camera(&mut self) {
        if let Some(motion) = self.mouse_motion {
            let mouse_delta = motion.get_delta();
            self.camera.borrow_mut().update_yaw_and_pitch(mouse_delta);
        }
    }

    pub fn update_scale(&self, distance: f32) {
        self.camera.borrow_mut().update_distance(distance);
    }

    fn screen_to_ndc(&self, screen_x: f32, screen_y: f32) -> (f32, f32)
    {
        let dim = self.context.get_framebuffer_dimensions();
        let (screen_width, screen_height) = (dim.0 as f32 / 2.0, dim.1 as f32 / 2.0);
        let ndc_x = (2.0 * screen_x / screen_width) - 1.0;
        let ndc_y = 1.0 - (2.0 * screen_y / screen_height);
        (ndc_x, ndc_y)
    }

    fn ndc_to_camera_space(&self, ndc_x: f32, ndc_y: f32) -> glm::Vec3 {
        let clip_coords = glm::vec4(ndc_x, ndc_y, -1.0, 1.0);
        let inv_proj_matrix = &self.projection_matrix.try_inverse().unwrap();
        let eye_coords = inv_proj_matrix * clip_coords;

        eye_coords.xyz() / eye_coords.w
    }

    fn ray_to(&self, x: f32, y: f32) -> Ray {
        let (ndc_x, ndc_y) = self.screen_to_ndc(x, y);
        let world_point = self.ndc_to_camera_space(ndc_x, ndc_y);
        Ray::new(self.camera.borrow().position, world_point)
    }

    pub fn is_model_clicked(&self, x: f32, y: f32) -> bool {
        // TODO optimization: this function should make bounds intersection check
        // for this approach we need to have bounds of every model object

        if let Some(HoverState::OnEmptyArea) = self.mouse_hover {
            return false;
        }
        let ray = self.ray_to(x, y);

        // --- CLICK ON THE CUBE ---
        let info = self.visible_objects
            .iter().map(|body_part| self.model_objects.get(body_part).unwrap())
            .flat_map(|obj| obj.cross(&ray))
            .min_by(|a, b| a.dist.total_cmp(&b.dist));

        info.is_some()
    }

    /// Returns the nearest clicked cell by screen coordinates.
    pub fn get_cell(&self, x: f32, y: f32, must_be_colored: bool) -> Option<ModelCell> {
        let ray = self.ray_to(x, y);
        let mut clicked_cell: Option<(ModelCell, f32)> = None;
        for body_part in self.visible_objects.iter() {
            let model_object = self.model_objects.get(body_part).unwrap();
            let cross = match model_object.cross(&ray) {
                Some(value) => value,
                None => continue
            };

            // check if the color is transparent
            if must_be_colored && model_object.get_pixel(cross.cell_index) == [0.0, 0.0, 0.0, 0.0] {
                continue;
            }

            match clicked_cell {
                Some((_, other_cross_distance)) => {
                    if cross.dist < other_cross_distance {
                        let cell = ModelCell {
                            body_part: body_part.clone(),
                            cell_index: cross.cell_index,
                            color: model_object.get_pixel(cross.cell_index),
                        };
                        clicked_cell = Some((cell, cross.dist))
                    }
                },
                None => {
                    let cell = ModelCell {
                        body_part: body_part.clone(),
                        cell_index: cross.cell_index,
                        color: model_object.get_pixel(cross.cell_index),
                    };
                    clicked_cell = Some((cell, cross.dist));
                }
            }
        }


        return match clicked_cell {
            Some((cell, _)) => Some(cell),
            None => None
        };
    }

    pub fn set_cell(&mut self, cell: &ModelCell) {
        let mut model_object = self.model_objects.get_mut(&cell.body_part).unwrap();
        model_object.paint(cell.cell_index, cell.color);
    }

    pub fn paint(&mut self, x: f32, y: f32, color: [f32; 4]) {
        let clicked_cell = self.get_cell(x, y, false);
        if let Some(cell) = clicked_cell {
            if self.visible_objects.contains(&cell.body_part) {
                self.model_objects.get_mut(&cell.body_part).unwrap().paint(cell.cell_index, color);
            }
        }
    }
    
    pub fn export_texture(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let width = 64;
        let height = 64;
        let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

        let parser = SkinParser::new();
        for (body_part, cell_object) in &self.model_objects {
            parser.export_as(&body_part, &mut imgbuf, &cell_object.get_vertexes());
        }

        imgbuf
    }

    pub fn set_body_part_active(&mut self, body_part: &BodyPart, visible: bool) {
        if visible {
            self.visible_objects.insert(body_part.clone());
            self.visible_objects.insert(body_part.clone());
        } else {
            self.visible_objects.remove(body_part);
            self.visible_objects.remove(body_part);
        }
    }
}