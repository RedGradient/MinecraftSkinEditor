use std::hash::Hash;

use glium::implement_vertex;

implement_vertex!(Vertex, position, color);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

implement_vertex!(VertexTex, position, tex_coords, face_id);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VertexTex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    face_id: i32,
}

impl VertexTex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2], face_id: i32) -> VertexTex {
        VertexTex { position, tex_coords, face_id }
    }
}