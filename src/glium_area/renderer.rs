use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::ops::Range;
use std::rc::Rc;

use glium::{Frame, Program, Surface};
use glium::backend::Context;
use gtk::gio;
use gtk::gio::ResourceLookupFlags;
use image::{ImageBuffer, Rgba};
use nalgebra_glm as glm;
use nalgebra_glm::Mat4;

use CubeSide::*;

use crate::glium_area::body_part::BodyPart;
use crate::glium_area::body_part::BodyPart::*;
use crate::glium_area::camera::Camera;
use crate::glium_area::cube_side::CubeSide;
use crate::glium_area::hover::Hover;
use crate::glium_area::model;
use crate::glium_area::model::{arm_fn, body_fn, generate_indexes, head_fn, head_grid};
use crate::glium_area::model::arm_fn::{cuboid_3x12x4, cuboid_4x12x4, grid_3x12x4, grid_4x12x4};
use crate::glium_area::model_object::{ModelIndexType, ModelObject};
use crate::glium_area::mouse_move::MouseMove;
use crate::glium_area::ray::Ray;
use crate::glium_area::skin_parser::{ModelType, SkinParser};
use crate::glium_area::vertex::Vertex;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ModelCell {
    pub body_part: BodyPart,
    pub cell_index: usize,
    pub color: [f32; 4],
}


pub struct Renderer {
    context: Rc<Context>,
    program: Rc<Program>,
    camera: Rc<RefCell<Camera>>,
    mouse_motion: Option<MouseMove>,
    mouse_move_on_model: bool,
    projection_matrix: Mat4,
    transform_matrix: Mat4,
    model_objects: BTreeMap<BodyPart, ModelObject>,
    visible_objects: BTreeSet<BodyPart>,
    current_color: glm::Vec4,
    mouse_hover: Option<Hover>,

    grid: bool,
    grid_objects: BTreeMap<BodyPart, ModelObject>,

    model_type: ModelType
}


const BACKGROUND_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);
const INNER_SCALE: glm::Vec3 = glm::Vec3::new(1.0, 1.0, 1.0);
const OUTER_SCALE: glm::Vec3 = glm::Vec3::new(1.15, 1.15, 1.15);
const GRID_SCALE: f32 = 1.005;


impl Renderer {
    fn create_model_objects(context: Rc<Context>, program: Rc<Program>, camera: Rc<RefCell<Camera>>, model_type: &ModelType) -> BTreeMap<BodyPart, ModelObject> {
        let factory = ModelObjectFactory::new(context.clone(), program.clone(), camera.clone());
        
        let head = factory.create_body_part(&head_fn::head_vertexes(), &glm::Vec3::new(0., 1.5, 0.), &INNER_SCALE);
        let body = factory.create_body_part(&body_fn::body_vertexes(), &glm::Vec3::new(0., 0.25, 0.), &INNER_SCALE);
        let right_leg = factory.create_body_part(&arm_fn::cuboid_4x12x4(), &glm::Vec3::new(-0.25, -1.25, 0.), &INNER_SCALE);
        let left_leg = factory.create_body_part(&arm_fn::cuboid_4x12x4(), &glm::Vec3::new(0.25, -1.25, 0.), &INNER_SCALE);
        let head_outer = factory.create_body_part(&head_fn::head_vertexes(), &glm::Vec3::new(0.0, 1.5, 0.0), &OUTER_SCALE);
        let body_outer = factory.create_body_part(&body_fn::body_vertexes(), &glm::Vec3::new(0., 0.25, 0.), &OUTER_SCALE);
        let right_leg_outer = factory.create_body_part(&arm_fn::cuboid_4x12x4(), &glm::Vec3::new(-0.25, -1.25, 0.), &OUTER_SCALE);
        let left_leg_outer = factory.create_body_part(&arm_fn::cuboid_4x12x4(), &glm::Vec3::new(0.25, -1.25, 0.), &OUTER_SCALE);
        
        let mut model_objects: BTreeMap<BodyPart, ModelObject> = BTreeMap::new();
        model_objects.insert(BodyPart::Head, head);
        model_objects.insert(BodyPart::Torso, body);
        model_objects.insert(BodyPart::RightLeg, right_leg);
        model_objects.insert(BodyPart::LeftLeg, left_leg);
        model_objects.insert(BodyPart::HeadOuter, head_outer);
        model_objects.insert(BodyPart::TorsoOuter, body_outer);
        model_objects.insert(BodyPart::RightLegOuter, right_leg_outer);
        model_objects.insert(BodyPart::LeftLegOuter, left_leg_outer);

        model_objects.extend(
            Renderer::get_arms(context.clone(), program.clone(), camera.clone(), model_type)
        );

        model_objects
    }
    fn create_grid_objects(context: Rc<Context>, program: Rc<Program>, camera: Rc<RefCell<Camera>>, model_type: &ModelType) -> BTreeMap<BodyPart, ModelObject> {
        let factory = ModelObjectFactory::new(context.clone(), program.clone(), camera.clone());

        let head_grid = factory.create_grid(&model::head_grid(), &glm::Vec3::new(0., 1.5, 0.), &INNER_SCALE.scale(GRID_SCALE));
        let body_grid = factory.create_grid(&body_fn::body_grid(), &glm::Vec3::new(0., 0.25, 0.), &INNER_SCALE.scale(GRID_SCALE));
        let right_leg_grid = factory.create_grid(&arm_fn::grid_4x12x4(), &glm::Vec3::new(-0.25, -1.25, 0.), &INNER_SCALE.scale(GRID_SCALE));
        let left_leg_grid = factory.create_grid(&arm_fn::grid_4x12x4(), &glm::Vec3::new(0.25, -1.25, 0.), &INNER_SCALE.scale(GRID_SCALE));
        let head_outer_grid = factory.create_grid(&model::head_grid(), &glm::Vec3::new(0.0, 1.5, 0.0), &OUTER_SCALE.scale(GRID_SCALE));
        let body_outer_grid = factory.create_grid(&body_fn::body_grid(), &glm::Vec3::new(0., 0.25, 0.), &OUTER_SCALE.scale(GRID_SCALE));
        let right_leg_outer_grid = factory.create_grid(&arm_fn::grid_4x12x4(), &glm::Vec3::new(-0.25, -1.25, 0.), &OUTER_SCALE.scale(GRID_SCALE));
        let left_leg_outer_grid = factory.create_grid(&arm_fn::grid_4x12x4(), &glm::Vec3::new(0.25, -1.25, 0.), &OUTER_SCALE.scale(GRID_SCALE));
        
        let mut grid_objects: BTreeMap<BodyPart, ModelObject> = BTreeMap::new();
        grid_objects.insert(BodyPart::Head, head_grid);
        grid_objects.insert(BodyPart::Torso, body_grid);
        grid_objects.insert(BodyPart::RightLeg, right_leg_grid);
        grid_objects.insert(BodyPart::LeftLeg, left_leg_grid);
        grid_objects.insert(BodyPart::HeadOuter, head_outer_grid);
        grid_objects.insert(BodyPart::TorsoOuter, body_outer_grid);
        grid_objects.insert(BodyPart::RightLegOuter, right_leg_outer_grid);
        grid_objects.insert(BodyPart::LeftLegOuter, left_leg_outer_grid);

        grid_objects.extend(
            Renderer::get_arm_grids(context.clone(), program.clone(), camera.clone(), model_type)
        );

        grid_objects
    }

    pub fn new(context: Rc<glium::backend::Context>) -> Self {
        let vertex_shader = {
            let bytes = gio::resources_lookup_data("/io/redgradient/MCSkinEditor/vertex.glsl", ResourceLookupFlags::NONE)
                .expect("Failed to get vertex shader");
            String::from_utf8(bytes.to_vec()).expect("Failed to get vertex shader")
        };
        let fragment_shader = {
            let bytes = gio::resources_lookup_data("/io/redgradient/MCSkinEditor/fragment.glsl", ResourceLookupFlags::NONE)
                .expect("Failed to get fragment shader");
            String::from_utf8(bytes.to_vec()).expect("Failed to get fragment shader")
        };
        let program = glium::Program::from_source(
            &context,
            vertex_shader.as_str(),
            fragment_shader.as_str(),
            None,
        ).unwrap();
        let program = Rc::new(program);
        let camera = Rc::new(RefCell::new(Camera::new()));
        let projection_matrix = glm::Mat4::identity();
        let model_type = ModelType::Slim;
        let model_objects = Renderer::create_model_objects(
            context.clone(), program.clone(), camera.clone(), &model_type);
        let grid_objects = Renderer::create_grid_objects(
            context.clone(), program.clone(), camera.clone(), &model_type);
        let mut visible_objects = BTreeSet::from([
            Head, Torso, RightArm, LeftArm, RightLeg, LeftLeg,
            HeadOuter, TorsoOuter, RightArmOuter, LeftArmOuter, RightLegOuter, LeftLegOuter
        ]);
        let current_color = glm::Vec4::new(0., 0., 1., 1.);

        Renderer {
            context,
            program,
            camera,
            mouse_motion: None,
            mouse_move_on_model: false,
            projection_matrix,
            transform_matrix: Mat4::identity(),
            model_objects,
            visible_objects,
            current_color,
            mouse_hover: None,

            grid: true,
            grid_objects,

            model_type
        }
    }

    fn get_arms(context: Rc<Context>, program: Rc<Program>, camera: Rc<RefCell<Camera>>, model_type: &ModelType) -> BTreeMap<BodyPart, ModelObject> {
        let factory = ModelObjectFactory::new(context.clone(), program.clone(), camera.clone());
        let mut arms: BTreeMap<BodyPart, ModelObject> = BTreeMap::new();

        let (vertexes, translation_x) = match model_type {
            ModelType::Classic => (cuboid_4x12x4(), 0.75),
            ModelType::Slim => (cuboid_3x12x4(), 0.6875)
        };

        let right_arm = factory.create_body_part(&vertexes, &glm::Vec3::new(-translation_x, 0.25, 0.), &INNER_SCALE);
        let left_arm = factory.create_body_part(&vertexes, &glm::Vec3::new(translation_x, 0.25, 0.), &INNER_SCALE);
        let right_arm_outer = factory.create_body_part(&vertexes, &glm::Vec3::new(-translation_x, 0.25, 0.), &OUTER_SCALE);
        let left_arm_outer = factory.create_body_part(&vertexes, &glm::Vec3::new(translation_x, 0.25, 0.), &OUTER_SCALE);

        arms.insert(BodyPart::RightArm, right_arm);
        arms.insert(BodyPart::LeftArm, left_arm);
        arms.insert(BodyPart::RightArmOuter, right_arm_outer);
        arms.insert(BodyPart::LeftArmOuter, left_arm_outer);

        arms
    }

    fn get_arm_grids(context: Rc<Context>, program: Rc<Program>, camera: Rc<RefCell<Camera>>, model_type: &ModelType) -> BTreeMap<BodyPart, ModelObject>{
        let factory = ModelObjectFactory::new(context.clone(), program.clone(), camera.clone());
        let mut grids: BTreeMap<BodyPart, ModelObject> = BTreeMap::new();
        let translation_classic = 0.75;
        let translation_slim = 0.6875;
        let (vertexes, translation_x) = match model_type {
            ModelType::Classic => (grid_4x12x4(), translation_classic),
            ModelType::Slim => (grid_3x12x4(), translation_slim)
        };
        
        let right_arm_grid = factory.create_grid(&vertexes, &glm::Vec3::new(-translation_x, 0.25, 0.), &INNER_SCALE.scale(GRID_SCALE));
        let left_arm_grid = factory.create_grid(&vertexes, &glm::Vec3::new(translation_x, 0.25, 0.), &INNER_SCALE.scale(GRID_SCALE));
        let right_arm_grid_outer = factory.create_grid(&vertexes, &glm::Vec3::new(-translation_x, 0.25, 0.), &OUTER_SCALE.scale(GRID_SCALE));
        let left_arm_grid_outer = factory.create_grid(&vertexes, &glm::Vec3::new(translation_x, 0.25, 0.), &OUTER_SCALE.scale(GRID_SCALE));
        
        grids.insert(BodyPart::RightArm, right_arm_grid);
        grids.insert(BodyPart::LeftArm, left_arm_grid);
        grids.insert(BodyPart::RightArmOuter, right_arm_grid_outer);
        grids.insert(BodyPart::LeftArmOuter, left_arm_grid_outer);

        grids
    }

    pub fn reset_model_type(&mut self, model_type: &ModelType) {
        let arms = Renderer::get_arms(self.context.clone(), self.program.clone(), self.camera.clone(), model_type);
        let grids = Renderer::get_arm_grids(self.context.clone(), self.program.clone(), self.camera.clone(), model_type);

        self.model_objects.extend(arms);
        self.grid_objects.extend(grids);

        self.model_type = model_type.clone();
    }

    pub fn set_grid_show(&mut self, show: bool) {
        self.grid = show;
    }

    pub fn load_texture(&mut self, path: &str, model_type: &ModelType) {

        let parser = SkinParser::new(model_type);
        let color_map = parser.load(path).unwrap();

        for (body_part, model_object) in self.model_objects.iter_mut() {
            if let Some(color_map) = color_map.get(body_part) {
                model_object.set_pixels(color_map);
            }
        }
    }

    pub fn get_mouse_hover(&self) -> Option<Hover> {
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

        frame.clear_color_and_depth(BACKGROUND_COLOR, 1.0);

        for body_part in &self.visible_objects {
            if self.grid {
                self.grid_objects.get_mut(body_part).expect("Some grid part is missed").draw(&mut frame);
            }
            self.model_objects.get_mut(body_part).expect("Some body part is missed").draw(&mut frame);
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

    pub fn set_mouse_hover(&mut self, hover: Option<Hover>) { self.mouse_hover = hover; }

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

        if let Some(Hover::OnEmptyArea) = self.mouse_hover {
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

    pub fn get_side_cells(&self, body_part: &BodyPart, cell_index: usize) -> Option<Vec<ModelCell>> {

        let cell_count_per_side: [usize; 6] = match body_part {
            Head | HeadOuter => [64, 64, 64, 64, 64, 64],
            Torso | TorsoOuter => [96, 48, 96, 48, 32, 32],
            RightArm | LeftArm | RightArmOuter | LeftArmOuter => match self.model_type {
                ModelType::Slim => [36, 48, 36, 48, 12, 12],
                ModelType::Classic => [48, 48, 48, 48, 16, 16]
            },
            RightLeg | LeftLeg | RightLegOuter | LeftLegOuter => [48, 48, 48, 48, 16, 16]
        };

        if cell_index >= cell_count_per_side.iter().sum() {
            return None;
        }

        let mut range: Option<Range<usize>> = None;

        let mut start = 0;
        for count in cell_count_per_side {
            let end = start + count;
            if (start..end).contains(&cell_index) {
                range.replace((start..end));
            }
            start = end;
        }

        let vertices: Vec<Vertex> = self.model_objects.get(&body_part)?.get_vertexes();

        let result: Vec<ModelCell> = range?.map(|index| ModelCell {
            body_part: body_part.clone(),
            cell_index: index,
            color: vertices[index * 4].color,
        }).collect();

        Some(result)
    }

    pub fn export_texture(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let width = 64;
        let height = 64;
        let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

        let parser = SkinParser::new(&self.model_type);
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


struct ModelObjectFactory {
    context: Rc<Context>,
    program: Rc<Program>,
    camera: Rc<RefCell<Camera>>,
}
impl ModelObjectFactory {
    pub fn new(context: Rc<Context>, program: Rc<Program>, camera: Rc<RefCell<Camera>>) -> ModelObjectFactory {
        ModelObjectFactory { context, program, camera }
    }

    fn create_body_part(&self, vertexes: &[Vertex], translation: &glm::Vec3, scale: &glm::Vec3) -> ModelObject {
        ModelObject::new(
            self.context.clone(),
            self.program.clone(),
            self.camera.clone(),
            vertexes,
            ModelIndexType::TrianglesList(generate_indexes(vertexes.len() / 4)),
            translation,
            scale
        )
    }

    fn create_grid(&self, vertexes: &[Vertex], translation: &glm::Vec3, scale: &glm::Vec3) -> ModelObject {
        ModelObject::new(
            self.context.clone(),
            self.program.clone(),
            self.camera.clone(),
            vertexes,
            ModelIndexType::LinesList,
            translation,
            scale
        )
    }
}