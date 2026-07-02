use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::Hash;
use std::ops::Range;
use std::rc::Rc;

use glium::{DrawParameters, Frame, IndexBuffer, Program, Rect, Surface, Texture2d, uniform, VertexBuffer};
use glium::draw_parameters::{BackfaceCullingMode, Depth, DepthTest};
use glium::backend::Context;
use glium::index::PrimitiveType;
use glium::texture::RawImage2d;
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
use crate::glium_area::model::{
    body_grid, body_vertices, cuboid_3x12x4, cuboid_4x12x4, grid_3x12x4, grid_4x12x4, head_grid,
    head_vertices, BODY_CELLS_PER_SIDE, HEAD_CELLS_PER_SIDE, LIMB_3_CELLS_PER_SIDE,
    LIMB_4_CELLS_PER_SIDE,
};
use crate::glium_area::model_object::{ModelDrawPass, ModelObject, ModelObjectType};
use crate::glium_area::mouse_move::MouseMove;
use crate::glium_area::ray::Ray;
use crate::glium_area::skin_parser::{ColorMap, ModelType, SkinParser, TextureLoadError, TextureType};
use crate::glium_area::vertex::{Vertex, VertexTex};
use crate::utils;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ModelCell {
    pub body_part: BodyPart,
    pub cell_index: usize,
    pub color: [f32; 4],
}
impl ModelCell {
    pub fn same_cell(&self, other: ModelCell) -> bool {
        self.body_part == other.body_part && self.cell_index == other.cell_index
    }
}

pub enum Side {
    Right,
    Left
}

struct FaceIndicator {
    context: Rc<Context>,
    camera: Rc<RefCell<Camera>>,
    vertex_buffer: VertexBuffer<VertexTex>,
    index_buffer: IndexBuffer<u8>,
    program: Program,

    front_texture: Texture2d,
    back_texture: Texture2d,
    right_texture: Texture2d,
    left_texture: Texture2d,
    top_texture: Texture2d,
    bottom_texture: Texture2d,
}

impl FaceIndicator {
    pub fn new(context: Rc<Context>, camera: Rc<RefCell<Camera>>) -> FaceIndicator {
        let vertex_shader = {
            let path = "/io/redgradient/MCSkinEditor/shaders/face-indicator/vertex.glsl";
            let bytes = gio::resources_lookup_data(path, ResourceLookupFlags::NONE)
                .expect("Failed to get vertex shader");
            String::from_utf8(bytes.to_vec()).expect("Failed to get vertex shader")
        };
        let fragment_shader = {
            let path = "/io/redgradient/MCSkinEditor/shaders/face-indicator/fragment.glsl";
            let bytes = gio::resources_lookup_data(path, ResourceLookupFlags::NONE)
                .expect("Failed to get fragment shader");
            String::from_utf8(bytes.to_vec()).expect("Failed to get fragment shader")
        };
        let vertex_buffer = FaceIndicator::get_vertices(context.clone());
        let index_buffer = FaceIndicator::get_indices(context.clone());
        let program = Program::from_source(
            &context,
            vertex_shader.as_str(),
            fragment_shader.as_str(),
            None
        ).unwrap();
        
        let front_texture = FaceIndicator::load_texture(context.clone(), Front);
        let back_texture = FaceIndicator::load_texture(context.clone(), Back);
        let right_texture = FaceIndicator::load_texture(context.clone(), Right);
        let left_texture = FaceIndicator::load_texture(context.clone(), Left);
        let top_texture = FaceIndicator::load_texture(context.clone(), Top);
        let bottom_texture = FaceIndicator::load_texture(context.clone(), Bottom);

        FaceIndicator {
            context,
            camera,
            vertex_buffer,
            index_buffer,
            program,

            front_texture,
            back_texture,
            right_texture,
            left_texture,
            top_texture,
            bottom_texture,
        }
    }

    fn get_vertices(context: Rc<Context>) -> VertexBuffer<VertexTex> {
        let vertices = vec![
            // Front face
            VertexTex::new([-0.5, -0.5, 0.5], [0.0, 0.0], 0),
            VertexTex::new([0.5, -0.5, 0.5], [1.0, 0.0], 0),
            VertexTex::new([0.5, 0.5, 0.5], [1.0, 1.0], 0),
            VertexTex::new([-0.5, 0.5, 0.5], [0.0, 1.0], 0),
    
            // Back face
            VertexTex::new([-0.5, -0.5, -0.5], [0.0, 0.0], 1),
            VertexTex::new([0.5, -0.5, -0.5], [1.0, 0.0], 1),
            VertexTex::new([0.5, 0.5, -0.5], [1.0, 1.0], 1),
            VertexTex::new([-0.5, 0.5, -0.5], [0.0, 1.0], 1),
    
            // Top face
            VertexTex::new([-0.5, 0.5, 0.5], [0.0, 0.0], 2),
            VertexTex::new([0.5, 0.5, 0.5], [1.0, 0.0], 2),
            VertexTex::new([0.5, 0.5, -0.5], [1.0, 1.0], 2),
            VertexTex::new([-0.5, 0.5, -0.5], [0.0, 1.0], 2),
    
            // Bottom face
            VertexTex::new([-0.5, -0.5, 0.5], [1.0, 1.0], 3),
            VertexTex::new([0.5, -0.5, 0.5], [0.0, 1.0], 3),
            VertexTex::new([0.5, -0.5, -0.5], [0.0, 0.0], 3),
            VertexTex::new([-0.5, -0.5, -0.5], [1.0, 0.0], 3),
    
            // Right face
            VertexTex::new([0.5, -0.5, 0.5], [1.0, 0.0], 4),
            VertexTex::new([0.5, -0.5, -0.5], [0.0, 0.0], 4),
            VertexTex::new([0.5, 0.5, -0.5], [0.0, 1.0], 4),
            VertexTex::new([0.5, 0.5, 0.5], [1.0, 1.0], 4),
    
            // Left face
            VertexTex::new([-0.5, -0.5, 0.5], [0.0, 0.0], 5),
            VertexTex::new([-0.5, -0.5, -0.5], [1.0, 0.0], 5),
            VertexTex::new([-0.5, 0.5, -0.5], [1.0, 1.0], 5),
            VertexTex::new([-0.5, 0.5, 0.5], [0.0, 1.0], 5),
        ];
        
        VertexBuffer::new(&context, &vertices).unwrap()
    }
    fn get_indices(context: Rc<Context>) -> IndexBuffer<u8> {
        let indices: Vec<u8> = vec![
            0, 1, 2, 0, 2, 3,       // front
            4, 5, 6, 4, 6, 7,       // back
            8, 9, 10, 8, 10, 11,    // top
            12, 13, 14, 12, 14, 15, // bottom
            16, 17, 18, 16, 18, 19, // right
            20, 21, 22, 20, 22, 23, // left
        ];
        
        IndexBuffer::new(&context, PrimitiveType::TrianglesList, &indices).unwrap()
    }
    fn load_texture(context: Rc<Context>, side: CubeSide) -> Texture2d {
        let path = match side {
            Front =>  "/io/redgradient/MCSkinEditor/steve-front.png",
            Back =>   "/io/redgradient/MCSkinEditor/steve-back.png",
            Right =>  "/io/redgradient/MCSkinEditor/steve-right.png",
            Left =>   "/io/redgradient/MCSkinEditor/steve-left.png",
            Top =>    "/io/redgradient/MCSkinEditor/steve-top.png",
            Bottom => "/io/redgradient/MCSkinEditor/steve-bottom.png",
        };
        let bytes = gio::resources_lookup_data(path, ResourceLookupFlags::NONE)
            .expect("Unable to load texture for face indicator"); 
        let image = image::load_from_memory(bytes.as_ref()).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        Texture2d::new(&context, image).unwrap()
    }

    const VIEWPORT_FRACTION: f32 = 0.11;
    const MARGIN_FRACTION: f32 = 0.015;
    /// Fills ~45% of the square corner viewport (matches the old full-frame size).
    const VIEWPORT_CUBE_SCALE: f32 = 0.85;

    fn corner_viewport(context: &Rc<Context>) -> Rect {
        let (fb_w, fb_h) = context.get_framebuffer_dimensions();
        let margin = (fb_h as f32 * Self::MARGIN_FRACTION).round() as u32;
        let size = (fb_h as f32 * Self::VIEWPORT_FRACTION).round().max(32.0) as u32;

        Rect {
            left: fb_w.saturating_sub(size + margin),
            bottom: fb_h.saturating_sub(size + margin),
            width: size,
            height: size,
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let viewport = Self::corner_viewport(&self.context);

        // Local ortho in the corner viewport; rotation matches the main model.
        let projection_matrix = glm::ortho(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);
        let scale_matrix = glm::scale(
            &Mat4::identity(),
            &glm::vec3(
                Self::VIEWPORT_CUBE_SCALE,
                Self::VIEWPORT_CUBE_SCALE,
                Self::VIEWPORT_CUBE_SCALE,
            ),
        );
        let matrix = projection_matrix
            * self.camera.borrow().get_rotation_matrix()
            * scale_matrix;

        let draw_parameters = DrawParameters {
            viewport: Some(viewport),
            scissor: Some(viewport),
            depth: Depth {
                test: DepthTest::Overwrite,
                write: false,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            backface_culling: BackfaceCullingMode::CullingDisabled,
            ..Default::default()
        };

        // Mysterious hack. Without this line, the texture is not applied to the object
        let _ = Texture2d::empty(&self.context, 0, 0);

        let uniforms = uniform! {
            matrix: *matrix.as_ref(),
            front_tex: self.front_texture.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            back_tex: self.back_texture.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            right_tex: self.right_texture.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            left_tex: self.left_texture.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            top_tex: self.top_texture.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            bottom_tex: self.bottom_texture.sampled()
                .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        };

        frame.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniforms,
            &draw_parameters,
        ).unwrap();
    }
}

pub struct Renderer {
    context: Rc<Context>,
    program: Rc<Program>,
    camera: Rc<RefCell<Camera>>,
    mouse_motion: Option<MouseMove>,
    /// Widget size in logical pixels (matches GTK gesture coordinates).
    viewport_width: f32,
    viewport_height: f32,
    projection_matrix: Mat4,
    view_matrix: Mat4,
    model_objects: BTreeMap<BodyPart, ModelObject>,
    visible_objects: BTreeSet<BodyPart>,
    current_color: glm::Vec4,
    mouse_hover: Option<Hover>,

    grid: bool,
    grid_objects: BTreeMap<BodyPart, ModelObject>,

    model_type: ModelType,
    face_indicator: FaceIndicator,
}


const BACKGROUND_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);
const INNER_SCALE: glm::Vec3 = glm::Vec3::new(1.0, 1.0, 1.0);
const OUTER_SCALE: glm::Vec3 = glm::Vec3::new(1.15, 1.15, 1.15);
const GRID_SCALE: f32 = 1.005;


impl Renderer {
    fn create_model_objects(context: Rc<Context>, program: Rc<Program>, camera: Rc<RefCell<Camera>>, model_type: &ModelType) -> BTreeMap<BodyPart, ModelObject> {
        let factory = ModelObjectFactory::new(context.clone(), program.clone(), camera.clone());

        let head = factory.create_body_part(head_vertices(), &glm::vec3(0., 1.5, 0.), &INNER_SCALE, HEAD_CELLS_PER_SIDE);
        let body = factory.create_body_part(body_vertices(), &glm::vec3(0., 0.25, 0.), &INNER_SCALE, BODY_CELLS_PER_SIDE);
        let right_leg = factory.create_body_part(cuboid_4x12x4(), &glm::vec3(-0.25, -1.25, 0.), &INNER_SCALE, LIMB_4_CELLS_PER_SIDE);
        let left_leg = factory.create_body_part(cuboid_4x12x4(), &glm::vec3(0.25, -1.25, 0.), &INNER_SCALE, LIMB_4_CELLS_PER_SIDE);
        let head_outer = factory.create_body_part(head_vertices(), &glm::vec3(0.0, 1.5, 0.0), &OUTER_SCALE, HEAD_CELLS_PER_SIDE);
        let body_outer = factory.create_body_part(body_vertices(), &glm::vec3(0., 0.25, 0.), &OUTER_SCALE.scale(1.001), BODY_CELLS_PER_SIDE);
        let right_leg_outer = factory.create_body_part(cuboid_4x12x4(), &glm::vec3(-0.25, -1.25, 0.), &OUTER_SCALE.scale(1.0005), LIMB_4_CELLS_PER_SIDE);
        let left_leg_outer = factory.create_body_part(cuboid_4x12x4(), &glm::vec3(0.25, -1.25, 0.), &OUTER_SCALE, LIMB_4_CELLS_PER_SIDE);

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

        let head_grid_mesh = factory.create_grid(head_grid(), &glm::vec3(0., 1.5, 0.), &INNER_SCALE.scale(GRID_SCALE));
        let body_grid_mesh = factory.create_grid(body_grid(), &glm::vec3(0., 0.25, 0.), &INNER_SCALE.scale(GRID_SCALE));
        let right_leg_grid = factory.create_grid(grid_4x12x4(), &glm::vec3(-0.25, -1.25, 0.), &INNER_SCALE.scale(GRID_SCALE));
        let left_leg_grid = factory.create_grid(grid_4x12x4(), &glm::vec3(0.25, -1.25, 0.), &INNER_SCALE.scale(GRID_SCALE));
        let head_outer_grid = factory.create_grid(head_grid(), &glm::vec3(0.0, 1.5, 0.0), &OUTER_SCALE.scale(GRID_SCALE));
        let body_outer_grid = factory.create_grid(body_grid(), &glm::vec3(0., 0.25, 0.), &OUTER_SCALE.scale(GRID_SCALE).scale(1.001));
        let right_leg_outer_grid = factory.create_grid(grid_4x12x4(), &glm::vec3(-0.25, -1.25, 0.), &OUTER_SCALE.scale(GRID_SCALE).scale(1.0005));
        let left_leg_outer_grid = factory.create_grid(grid_4x12x4(), &glm::vec3(0.25, -1.25, 0.), &OUTER_SCALE.scale(GRID_SCALE));

        let mut grid_objects: BTreeMap<BodyPart, ModelObject> = BTreeMap::new();
        grid_objects.insert(BodyPart::Head, head_grid_mesh);
        grid_objects.insert(BodyPart::Torso, body_grid_mesh);
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

    pub fn new(context: Rc<Context>) -> Self {
        let vertex_shader = {
            let path = "/io/redgradient/MCSkinEditor/shaders/vertex.glsl";
            let bytes = gio::resources_lookup_data(path, ResourceLookupFlags::NONE)
                .expect("Failed to get vertex shader");
            String::from_utf8(bytes.to_vec()).expect("Failed to get vertex shader")
        };
        let fragment_shader = {
            let path = "/io/redgradient/MCSkinEditor/shaders/fragment.glsl";
            let bytes = gio::resources_lookup_data(path, ResourceLookupFlags::NONE)
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
        let view_matrix = glm::Mat4::identity();
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
        let face_indicator = FaceIndicator::new(context.clone(), camera.clone());

        Renderer {
            context,
            program,
            camera,
            mouse_motion: None,
            viewport_width: 1.0,
            viewport_height: 1.0,
            projection_matrix,
            view_matrix,
            model_objects,
            visible_objects,
            current_color,
            mouse_hover: None,

            grid: true,
            grid_objects,

            model_type,
            face_indicator,
        }
    }

    fn get_arms(context: Rc<Context>, program: Rc<Program>, camera: Rc<RefCell<Camera>>, model_type: &ModelType) -> BTreeMap<BodyPart, ModelObject> {
        let factory = ModelObjectFactory::new(context.clone(), program.clone(), camera.clone());
        let mut arms: BTreeMap<BodyPart, ModelObject> = BTreeMap::new();

        let (vertexes, translation_x, cells_per_side) = match model_type {
            ModelType::Classic => (cuboid_4x12x4(), 0.75, LIMB_4_CELLS_PER_SIDE),
            ModelType::Slim => (cuboid_3x12x4(), 0.6875, LIMB_3_CELLS_PER_SIDE),
        };

        let right_arm = factory.create_body_part(vertexes, &glm::Vec3::new(-translation_x, 0.25, 0.), &INNER_SCALE, cells_per_side);
        let left_arm = factory.create_body_part(vertexes, &glm::Vec3::new(translation_x, 0.25, 0.), &INNER_SCALE, cells_per_side);
        let right_arm_outer = factory.create_body_part(vertexes, &glm::Vec3::new(-translation_x, 0.25, 0.), &OUTER_SCALE, cells_per_side);
        let left_arm_outer = factory.create_body_part(vertexes, &glm::Vec3::new(translation_x, 0.25, 0.), &OUTER_SCALE, cells_per_side);

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
            ModelType::Slim => (grid_3x12x4(), translation_slim),
        };

        let right_arm_grid = factory.create_grid(vertexes, &glm::Vec3::new(-translation_x, 0.25, 0.), &INNER_SCALE.scale(GRID_SCALE));
        let left_arm_grid = factory.create_grid(vertexes, &glm::Vec3::new(translation_x, 0.25, 0.), &INNER_SCALE.scale(GRID_SCALE));
        let right_arm_grid_outer = factory.create_grid(vertexes, &glm::Vec3::new(-translation_x, 0.25, 0.), &OUTER_SCALE.scale(GRID_SCALE));
        let left_arm_grid_outer = factory.create_grid(vertexes, &glm::Vec3::new(translation_x, 0.25, 0.), &OUTER_SCALE.scale(GRID_SCALE));

        grids.insert(BodyPart::RightArm, right_arm_grid);
        grids.insert(BodyPart::LeftArm, left_arm_grid);
        grids.insert(BodyPart::RightArmOuter, right_arm_grid_outer);
        grids.insert(BodyPart::LeftArmOuter, left_arm_grid_outer);

        grids
    }

    pub fn reset_skin(&mut self) {
        for model_object in self.model_objects.values_mut() {
            model_object.clear();
        }
    }

    pub fn reset_model_type(&mut self, model_type: &ModelType) {
        if self.model_type == *model_type {
            return
        }
        
        let mut arms = Renderer::get_arms(self.context.clone(), self.program.clone(), self.camera.clone(), model_type);
        
        let parts = [RightArm, RightArmOuter, LeftArm, LeftArmOuter];
        for part in parts {
            let old_arm = self.model_objects.get(&part).unwrap();
            let new_arm = arms.get_mut(&part).unwrap();
            match model_type {
                ModelType::Slim => utils::classic_to_slim_arm(old_arm, new_arm, part),
                ModelType::Classic => utils::slim_to_classic_arm(old_arm, new_arm, part)
            }
        }
        
        let arm_grids = Renderer::get_arm_grids(self.context.clone(), self.program.clone(), self.camera.clone(), model_type);

        self.model_objects.extend(arms);
        self.grid_objects.extend(arm_grids);

        self.model_type = *model_type;
    }

    pub fn set_viewport_size(&mut self, width: i32, height: i32) {
        self.viewport_width = width.max(1) as f32;
        self.viewport_height = height.max(1) as f32;
    }

    pub fn set_grid_show(&mut self, show: bool) {
        self.grid = show;
    }

    fn projection_matrix_for_aspect(aspect: f32) -> Mat4 {
        let fov: f32 = std::f32::consts::PI / 3.0;
        let near = 0.1;
        let far = 1000.0;
        glm::perspective_rh(aspect.max(0.001), fov, near, far)
    }

    fn framebuffer_aspect(context: &Rc<Context>) -> f32 {
        let (width, height) = context.get_framebuffer_dimensions();
        width as f32 / height.max(1) as f32
    }

    fn sync_render_matrices(&mut self) {
        self.view_matrix = self.camera.borrow().get_view_matrix();
        self.projection_matrix =
            Self::projection_matrix_for_aspect(Self::framebuffer_aspect(&self.context));
    }

    fn sync_pick_matrices(&mut self) {
        self.sync_render_matrices();
    }

    pub fn get_model_type(&self) -> ModelType {
        self.model_type.clone()
    }

    pub fn load_texture(&mut self, path: &str, model_type: &ModelType, ignore_transparent: bool) -> Result<(), TextureLoadError> {
        let parser = SkinParser::new(model_type, TextureType::Normal);
        let color_map = parser.load_from_path(path)?;

        self.reset_model_type(&model_type);
        self.load_from_color_map(color_map, ignore_transparent);

        Ok(())
    }

    pub fn load_texture_from_bytes(&mut self,
                                   // bytes: &[u8],
                                   image: &image::DynamicImage, 
                                   model_type: ModelType,
                                   texture_type: TextureType,
                                   ignore_transparent: bool) -> Result<(), TextureLoadError>
    {
        let parser = SkinParser::new(&model_type, texture_type);
        let color_map = parser.load_from_bytes(image)?;
        
        self.reset_model_type(&model_type);
        self.load_from_color_map(color_map, ignore_transparent);

        Ok(())
    }

    fn load_from_color_map(&mut self, color_map: ColorMap, ignore_transparent: bool) {
        for (body_part, model_object) in self.model_objects.iter_mut() {
            if let Some(color_map) = color_map.get(body_part) {
                model_object.set_pixels(color_map, ignore_transparent);
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
        self.sync_render_matrices();

        let mut frame = Frame::new(
            self.context.clone(),
            self.context.get_framebuffer_dimensions(),
        );

        frame.clear_color_and_depth(BACKGROUND_COLOR, 1.0);

        for body_part in &self.visible_objects {
            if body_part.is_outer() {
                continue;
            }
            if self.grid {
                self.grid_objects
                    .get_mut(body_part)
                    .expect("Some grid part is missed")
                    .draw(&mut frame);
            }
            self.model_objects
                .get_mut(body_part)
                .expect("Some body part is missed")
                .draw(&mut frame);
        }

        for body_part in &self.visible_objects {
            if !body_part.is_outer() {
                continue;
            }
            let model_object = self
                .model_objects
                .get_mut(body_part)
                .expect("Some body part is missed");
            model_object.draw_pass(&mut frame, ModelDrawPass::OuterBackFaces);
            model_object.draw_pass(&mut frame, ModelDrawPass::OuterFrontFaces);
            if self.grid {
                self.grid_objects
                    .get_mut(body_part)
                    .expect("Some grid part is missed")
                    .draw(&mut frame);
            }
        }

        frame.clear_depth(1.0);
        self.face_indicator.draw(&mut frame);

        frame.finish().unwrap();
    }
    
    pub fn replace(&mut self, color_to_replace: [f32; 4], new_color: [f32; 4]) -> Vec<ModelCell> {
        let mut replaced_cells = vec![];
        for (body_part, model_object) in self.model_objects.iter_mut() {
            for (cell_index, &pixel) in model_object.get_pixels().iter().enumerate() {
                if pixel[0] == color_to_replace[0] && pixel[1] == color_to_replace[1] && pixel[2] == color_to_replace[2] {
                    let cell = ModelCell {
                        body_part: *body_part,
                        cell_index,
                        color: pixel
                    };
                    replaced_cells.push(cell);
                    model_object.paint(cell_index, new_color);
                }
            }
        }
        replaced_cells
    }

    pub fn mouse_move(&mut self, curr_x: f32, curr_y: f32) {
        if self.mouse_motion.is_some() {
            let mut mm = self.mouse_motion.take().unwrap();
            mm.move_to(curr_x, curr_y);
            self.mouse_motion = Some(mm);
        }
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

    fn screen_to_ndc(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
        let ndc_x = (2.0 * screen_x / self.viewport_width) - 1.0;
        let ndc_y = 1.0 - (2.0 * screen_y / self.viewport_height);
        (ndc_x, ndc_y)
    }

    fn ray_to(&self, x: f32, y: f32) -> Ray {
        let (ndc_x, ndc_y) = self.screen_to_ndc(x, y);
        let inv_vp = (self.projection_matrix * self.view_matrix)
            .try_inverse()
            .expect("view-projection matrix must be invertible");

        let unproject = |ndc_z: f32| {
            let clip = glm::vec4(ndc_x, ndc_y, ndc_z, 1.0);
            let world = inv_vp * clip;
            world.xyz() / world.w
        };

        let near = unproject(-1.0);
        let far = unproject(1.0);
        let direction = glm::normalize(&(far - near));
        Ray::new(near, direction)
    }

    /// Returns the closest clicked cell by screen coordinates.
    /// When an outer-layer part is visible, its inner counterpart is excluded from
    /// picking so overlay edits do not bleed into the body layer underneath.
    pub fn get_cell(&mut self, x: f32, y: f32, must_be_colored: bool) -> Option<ModelCell> {
        self.sync_pick_matrices();
        let ray = self.ray_to(x, y);
        let pickable = self.pickable_parts();
        self.pick_cell_on_parts(&ray, &pickable, must_be_colored)
    }

    fn pickable_parts(&self) -> Vec<BodyPart> {
        self.visible_objects
            .iter()
            .copied()
            .filter(|part| {
                part.outer_counterpart()
                    .is_none_or(|outer| !self.visible_objects.contains(&outer))
            })
            .collect()
    }

    fn pick_cell_on_parts(
        &self,
        ray: &Ray,
        parts: &[BodyPart],
        must_be_colored: bool,
    ) -> Option<ModelCell> {
        let mut closest: Option<(ModelCell, f32)> = None;

        for body_part in parts {
            let model_object = self.model_objects.get(body_part).unwrap();
            let cross = match model_object.cross(ray) {
                Some(value) => value,
                None => continue,
            };

            if must_be_colored && model_object.get_pixel(cross.cell_index)[3] == 0.0 {
                continue;
            }

            let cell = ModelCell {
                body_part: *body_part,
                cell_index: cross.cell_index,
                color: model_object.get_pixel(cross.cell_index),
            };

            let is_closer = closest
                .as_ref()
                .is_none_or(|(_, other_dist)| cross.dist < *other_dist);
            if is_closer {
                closest = Some((cell, cross.dist));
            }
        }

        closest.map(|(cell, _)| cell)
    }

    pub fn set_cell(&mut self, cell: &ModelCell) {
        let mut model_object = self.model_objects.get_mut(&cell.body_part).unwrap();
        model_object.paint(cell.cell_index, cell.color);
    }

    pub fn snapshot_cells(&self) -> BTreeMap<(BodyPart, usize), [f32; 4]> {
        let mut snapshot = BTreeMap::new();
        for (body_part, model_object) in &self.model_objects {
            for (cell_index, &color) in model_object.get_pixels().iter().enumerate() {
                snapshot.insert((body_part.clone(), cell_index), color);
            }
        }
        snapshot
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

        let parser = SkinParser::new(&self.model_type, TextureType::Normal);
        for (body_part, cell_object) in &self.model_objects {
            parser.export_as(&body_part, &mut imgbuf, &cell_object.get_vertexes());
        }

        imgbuf
    }

    pub fn set_body_part_active(&mut self, body_part: &BodyPart, visible: bool) {
        if visible {
            self.visible_objects.insert(body_part.clone());
        } else {
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

    fn create_body_part(
        &self,
        vertexes: &[Vertex],
        translation: &glm::Vec3,
        scale: &glm::Vec3,
        cells_per_side: [usize; 6],
    ) -> ModelObject {
        ModelObject::new(
            self.context.clone(),
            self.program.clone(),
            self.camera.clone(),
            vertexes,
            ModelObjectType::Model,
            translation,
            scale,
            cells_per_side,
        )
    }

    fn create_grid(&self, vertexes: &[Vertex], translation: &glm::Vec3, scale: &glm::Vec3) -> ModelObject {
        ModelObject::new(
            self.context.clone(),
            self.program.clone(),
            self.camera.clone(),
            vertexes,
            ModelObjectType::Grid,
            translation,
            scale,
            [0; 6],
        )
    }
}