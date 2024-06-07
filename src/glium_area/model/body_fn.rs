use crate::glium_area::model::{CELL_COLOR, CELL_SIZE, GRID_COLOR};
use crate::glium_area::vertex::Vertex;

const GRID_SIZE: f32 = 8.0;
const BODY_CELLS_COUNT: usize = 352;

fn front(vertices: &mut Vec<Vertex>) {
    let width = 8;
    let height = 12;

    for i in 0..height {
        for j in 0..width {
            let x = -0.5 + j as f32 * CELL_SIZE;
            let y = 0.75 - i as f32 * CELL_SIZE;
            let z = 0.25;

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
fn left(vertices: &mut Vec<Vertex>) {
    let width = 4;
    let height = 12;
    
    for i in 0..height {
        for j in 0..width {
            let x = 0.5;
            let y = 0.75 - i as f32 * CELL_SIZE;
            let z = 0.25 - j as f32 * CELL_SIZE;

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
fn back(vertices: &mut Vec<Vertex>) {
    let width = 8;
    let height = 12;

    for i in 0..height {
        for j in 0..width {
            let x = 0.5 - j as f32 * CELL_SIZE;
            let y = 0.75 - i as f32 * CELL_SIZE;
            let z = -0.25;

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
    let width = 4;
    let height = 12;

    for i in 0..height {
        for j in 0..width {
            let x = -0.5;
            let y = 0.75 - i as f32 * CELL_SIZE;
            let z = -0.25 + j as f32 * CELL_SIZE;

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
fn top(vertices: &mut Vec<Vertex>) {
    let width = 8;
    let height = 4;

    for i in 0..height {
        for j in 0..width {
            let x = -0.5 + j as f32 * CELL_SIZE;
            let y = 0.75;
            let z = -0.25 + i as f32 * CELL_SIZE;

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
    let width = 8;
    let height = 4;

    for i in 0..height {
        for j in 0..width {
            let x = -0.5 + j as f32 * CELL_SIZE;
            let y = -0.75;
            let z = -0.25 + i as f32 * CELL_SIZE;

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

pub fn body_vertices() -> Vec<Vertex> {
    let mut vertices = Vec::with_capacity(BODY_CELLS_COUNT);

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
fn front_grid(vertices: &mut Vec<Vertex>) {
    let width = 8 + 1;
    let height = 12 + 1;

    let z = 0.25;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = -0.5 + j as f32 * CELL_SIZE;

            // --- 2 VERTICAL LINES ---
            vertices.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- 2 HORIZONTAL LINES ---
        vertices.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }
}

fn left_grid(vertices: &mut Vec<Vertex>) {
    let width = 4 + 1;
    let height = 12 + 1;

    let x = 0.5;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let z = 0.25 - j as f32 * CELL_SIZE;

            // --- 2 VERTICAL LINES ---
            vertices.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- 2 HORIZONTAL LINES ---
        vertices.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
        vertices.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
    }
}

fn back_grid(vertices: &mut Vec<Vertex>) {
    let width = 8 + 1;
    let height = 12 + 1;

    let z = -0.25;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = 0.5 - j as f32 * CELL_SIZE;
            // --- 2 VERTICAL LINES ---
            vertices.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- 2 HORIZONTAL LINES ---
        vertices.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }
}

fn right_grid(vertices: &mut Vec<Vertex>) {
    let width = 4 + 1;
    let height = 12 + 1;

    let x = -0.5;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let z = -0.25 + j as f32 * CELL_SIZE;
            // --- 2 VERTICAL LINES ---
            vertices.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- 2 HORIZONTAL LINES ---
        vertices.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
        vertices.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
    }
}

fn top_grid(vertices: &mut Vec<Vertex>) {
    let width = 8 + 1;
    let height = 4 + 1;

    let y = 0.75;
    for i in 0..height {
        let z = -0.25 + (i as f32) * CELL_SIZE;
        for j in 0..width {
            let x = -0.5 + (j as f32) * CELL_SIZE;
            vertices.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
        }
        vertices.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }
}

fn bottom_grid(vertices: &mut Vec<Vertex>) {
    let width = 8 + 1;
    let height = 4 + 1;

    let y = -0.75;
    for i in 0..height {
        let z = -0.25 + i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = -0.5 + j as f32 * CELL_SIZE;
            vertices.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
        }
        vertices.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }
}


pub fn body_grid() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    front_grid(&mut vertices);
    left_grid(&mut vertices);
    back_grid(&mut vertices);
    right_grid(&mut vertices);
    top_grid(&mut vertices);
    bottom_grid(&mut vertices);

    vertices
}