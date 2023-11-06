use std::hash::Hash;

use glium::implement_vertex;

implement_vertex!(Vertex, position, color);

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}