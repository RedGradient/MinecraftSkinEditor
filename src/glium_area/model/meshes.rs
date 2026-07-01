use std::sync::OnceLock;

use crate::glium_area::model::obj_loader::{parse_cell_mesh, parse_line_mesh};
use crate::glium_area::vertex::Vertex;

const CELL_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 0.0];
const GRID_COLOR: [f32; 4] = [0.65, 0.65, 0.65, 1.0];

struct MeshLibrary {
    head: Vec<Vertex>,
    body: Vec<Vertex>,
    limb_4x12x4: Vec<Vertex>,
    limb_3x12x4: Vec<Vertex>,
    head_grid: Vec<Vertex>,
    body_grid: Vec<Vertex>,
    limb_4x12x4_grid: Vec<Vertex>,
    limb_3x12x4_grid: Vec<Vertex>,
}

static MESHES: OnceLock<MeshLibrary> = OnceLock::new();

fn library() -> &'static MeshLibrary {
    MESHES.get_or_init(|| {
        MeshLibrary {
            head: load_cell(include_str!("../../../resources/models/head.obj")),
            body: load_cell(include_str!("../../../resources/models/body.obj")),
            limb_4x12x4: load_cell(include_str!("../../../resources/models/limb_4x12x4.obj")),
            limb_3x12x4: load_cell(include_str!("../../../resources/models/limb_3x12x4.obj")),
            head_grid: load_grid(include_str!("../../../resources/models/head_grid.obj")),
            body_grid: load_grid(include_str!("../../../resources/models/body_grid.obj")),
            limb_4x12x4_grid: load_grid(include_str!(
                "../../../resources/models/limb_4x12x4_grid.obj"
            )),
            limb_3x12x4_grid: load_grid(include_str!(
                "../../../resources/models/limb_3x12x4_grid.obj"
            )),
        }
    })
}

fn load_cell(obj: &str) -> Vec<Vertex> {
    parse_cell_mesh(obj, CELL_COLOR).expect("embedded cell OBJ must be valid")
}

fn load_grid(obj: &str) -> Vec<Vertex> {
    parse_line_mesh(obj, GRID_COLOR).expect("embedded grid OBJ must be valid")
}

pub fn head_vertices() -> &'static [Vertex] {
    &library().head
}

pub fn body_vertices() -> &'static [Vertex] {
    &library().body
}

pub fn cuboid_4x12x4() -> &'static [Vertex] {
    &library().limb_4x12x4
}

pub fn cuboid_3x12x4() -> &'static [Vertex] {
    &library().limb_3x12x4
}

pub fn head_grid() -> &'static [Vertex] {
    &library().head_grid
}

pub fn body_grid() -> &'static [Vertex] {
    &library().body_grid
}

pub fn grid_4x12x4() -> &'static [Vertex] {
    &library().limb_4x12x4_grid
}

pub fn grid_3x12x4() -> &'static [Vertex] {
    &library().limb_3x12x4_grid
}

pub const HEAD_CELLS_PER_SIDE: [usize; 6] = [64, 64, 64, 64, 64, 64];
pub const BODY_CELLS_PER_SIDE: [usize; 6] = [96, 48, 96, 48, 32, 32];
pub const LIMB_4_CELLS_PER_SIDE: [usize; 6] = [48, 48, 48, 48, 16, 16];
pub const LIMB_3_CELLS_PER_SIDE: [usize; 6] = [36, 48, 36, 48, 12, 12];
