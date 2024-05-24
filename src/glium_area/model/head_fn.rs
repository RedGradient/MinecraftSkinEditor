use crate::glium_area::model::{CELL_COLOR, CELL_SIZE, GRID_COLOR};
use crate::glium_area::vertex::Vertex;

const GRID_SIZE: usize = 8;
const SIDE_LINE_COUNT: usize = 8 + 1;
pub(crate) const HEAD_CELLS_COUNT: usize = 384;

fn front(vertices: &mut Vec<Vertex>) {
    let z = 0.5;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = -0.5 + j as f32 * CELL_SIZE;
            let y = 0.5 - i as f32 * CELL_SIZE;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x + CELL_SIZE, y, z],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x + CELL_SIZE, y - CELL_SIZE, z],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x, y - CELL_SIZE, z],
                color: CELL_COLOR});
        }
    }
}
fn back(vertices: &mut Vec<Vertex>) {
    let z = -0.5;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = 0.5 - j as f32 * CELL_SIZE;
            let y = 0.5 - i as f32 * CELL_SIZE;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x - CELL_SIZE, y, z],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x - CELL_SIZE, y - CELL_SIZE, z],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x, y - CELL_SIZE, z],
                color: CELL_COLOR});
        }
    }
}
fn right(vertices: &mut Vec<Vertex>) {
    let x = -0.5;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let y = 0.5 - i as f32 * CELL_SIZE;
            let z = -0.5 + j as f32 * CELL_SIZE;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x, y, z + CELL_SIZE],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x, y - CELL_SIZE, z + CELL_SIZE],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x, y - CELL_SIZE, z],
                color: CELL_COLOR});
        }
    }
}
fn left(vertices: &mut Vec<Vertex>) {
    let x = 0.5;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let y = 0.5 - i as f32 * CELL_SIZE;
            let z = 0.5 - j as f32 * CELL_SIZE;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x, y, z - CELL_SIZE],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x, y - CELL_SIZE, z - CELL_SIZE],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x, y - CELL_SIZE, z],
                color: CELL_COLOR});
        }
    }
}
fn top(vertices: &mut Vec<Vertex>) {
    let y = 0.5;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = -0.5 + j as f32 * CELL_SIZE;
            let z = -0.5 + i as f32 * CELL_SIZE;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x + CELL_SIZE, y, z],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x + CELL_SIZE, y, z + CELL_SIZE],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x, y, z + CELL_SIZE],
                color: CELL_COLOR});
        }
    }
}
fn bottom(vertices: &mut Vec<Vertex>) {
    let y = -0.5;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = -0.5 + j as f32 * CELL_SIZE;
            let z = -0.5 + i as f32 * CELL_SIZE;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x, y, z + CELL_SIZE],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x + CELL_SIZE, y, z + CELL_SIZE],
                color: CELL_COLOR});
            vertices.push(Vertex {
                position: [x + CELL_SIZE, y, z],
                color: CELL_COLOR});
        }
    }
}

pub fn head_vertices() -> Vec<Vertex> {
    let mut vertices = Vec::with_capacity(HEAD_CELLS_COUNT);
    
    front(&mut vertices);
    left(&mut vertices);
    back(&mut vertices);
    right(&mut vertices);
    top(&mut vertices);
    bottom(&mut vertices);

    vertices
}


// ------------
// --- GRID ---
// ------------
pub(crate) fn front_grid(vertices: &mut Vec<Vertex>) {
    let z = 0.5;
    for i in 0..SIDE_LINE_COUNT {
        let y = 0.5 - i as f32 * CELL_SIZE;
        for j in 0..SIDE_LINE_COUNT {
            let x = -0.5 + j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, -0.5, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.5, z], color: GRID_COLOR});
        }

        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }
}
pub(crate) fn left_grid(vertices: &mut Vec<Vertex>) {
    let x = 0.5;
    for i in 0..SIDE_LINE_COUNT {
        let y = 0.5 - i as f32 * CELL_SIZE;
        for j in 0..SIDE_LINE_COUNT {
            let z = 0.5 - j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, -0.5, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.5, z], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [x, y, -0.5], color: GRID_COLOR});
        vertices.push(Vertex { position: [x, y, 0.5], color: GRID_COLOR});
    }
}
pub(crate) fn back_grid(vertices: &mut Vec<Vertex>) {
    let z = -0.5;
    for i in 0..SIDE_LINE_COUNT {
        let y = 0.5 - i as f32 * CELL_SIZE;
        for j in 0..SIDE_LINE_COUNT {
            let x = 0.5 - j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, -0.5, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.5, z], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }
}
pub(crate) fn right_grid(vertices: &mut Vec<Vertex>) {
    let x = -0.5;
    for i in 0..SIDE_LINE_COUNT {
        let y = 0.5 - i as f32 * CELL_SIZE;
        for j in 0..SIDE_LINE_COUNT {
            let z = -0.5 + j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, -0.5, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.5, z], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [x, y, -0.5], color: GRID_COLOR});
        vertices.push(Vertex { position: [x, y, 0.5], color: GRID_COLOR});
    }
}
pub(crate) fn top_grid(vertices: &mut Vec<Vertex>) {
    let y = 0.5;
    for i in 0..SIDE_LINE_COUNT {
        let z = -0.5 + i as f32 * CELL_SIZE;
        for j in 0..SIDE_LINE_COUNT {
            let x = -0.5 + j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, y, -0.5], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, y, 0.5], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }
}
pub(crate) fn bottom_grid(vertices: &mut Vec<Vertex>) {
    let y = -0.5;
    for i in 0..SIDE_LINE_COUNT {
        let z = -0.5 + i as f32 * CELL_SIZE;
        for j in 0..SIDE_LINE_COUNT {
            let x = -0.5 + j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, y, -0.5], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, y, 0.5], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }
}

pub fn head_grid() -> Vec<Vertex> {
    let mut vertices = Vec::with_capacity(SIDE_LINE_COUNT * SIDE_LINE_COUNT * 6);

    front_grid(&mut vertices);
    left_grid(&mut vertices);
    back_grid(&mut vertices);
    right_grid(&mut vertices);
    top_grid(&mut vertices);
    bottom_grid(&mut vertices);

    vertices
}
