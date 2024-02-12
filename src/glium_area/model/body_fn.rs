use crate::glium_area::model::{CELL_SIZE, generate_indexes, GRID_COLOR, CELL_COLOR};
use crate::glium_area::vertex::Vertex;

const GRID_SIZE: f32 = 8.0;
const BODY_CELLS_COUNT: usize = 352;

fn body_front() -> Vec<Vertex> {
    let mut vertices = Vec::new();

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

    vertices
}

fn body_left() -> Vec<Vertex> {
    let mut vertices = Vec::new();

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

    vertices
}

fn body_back() -> Vec<Vertex> {
    let mut vertices = Vec::new();

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

    vertices
}

fn body_right() -> Vec<Vertex> {
    let mut vertices = Vec::new();

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

    vertices
}

fn body_top() -> Vec<Vertex> {
    let mut vertices = Vec::new();

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

    vertices
}

fn body_bottom() -> Vec<Vertex> {
    let mut vertices = Vec::new();

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

    vertices
}


pub fn body_vertexes() -> Vec<Vertex> {
    let mut vertexes = Vec::with_capacity(BODY_CELLS_COUNT);

    vertexes.extend(body_front());
    vertexes.extend(body_left());
    vertexes.extend(body_back());
    vertexes.extend(body_right());
    vertexes.extend(body_top());
    vertexes.extend(body_bottom());

    vertexes
}

pub fn body_indexes() -> Vec<u16> {
    generate_indexes(BODY_CELLS_COUNT)
}


// ------------
// --- GRID ---
// ------------
fn body_front_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 8;
    let height = 12;

    let z = 0.25;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = -0.5 + j as f32 * CELL_SIZE;

            // --- 2 VERTICAL LINES ---
            grid.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            grid.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- 2 HORIZONTAL LINES ---
        grid.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        grid.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }

    grid
}

fn body_left_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 4;
    let height = 12;

    let x = 0.5;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let z = 0.25 - j as f32 * CELL_SIZE;

            // --- 2 VERTICAL LINES ---
            grid.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            grid.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- 2 HORIZONTAL LINES ---
        grid.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
        grid.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
    }

    grid
}

fn body_back_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 8;
    let height = 12;

    let z = -0.25;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = 0.5 - j as f32 * CELL_SIZE;
            // --- 2 VERTICAL LINES ---
            grid.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            grid.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- 2 HORIZONTAL LINES ---
        grid.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        grid.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }

    grid
}

fn body_right_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 4;
    let height = 12;

    let x = -0.5;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let z = -0.25 + j as f32 * CELL_SIZE;
            // --- 2 VERTICAL LINES ---
            grid.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            grid.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- 2 HORIZONTAL LINES ---
        grid.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
        grid.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
    }

    grid
}

fn body_top_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 8;
    let height = 4;

    let y = 0.75;
    for i in 0..height {
        let z = -0.25 + (i as f32) * CELL_SIZE;
        for j in 0..width {
            let x = -0.5 + (j as f32) * CELL_SIZE;
            grid.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
            grid.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
        }
        grid.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        grid.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }

    grid
}

fn body_bottom_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 8;
    let height = 4;

    let y = -0.75;
    for i in 0..height {
        let z = -0.25 + i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = -0.5 + j as f32 * CELL_SIZE;
            grid.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
            grid.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
        }
        grid.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        grid.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }

    grid
}


pub fn body_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    grid.extend(body_front_grid());
    grid.extend(body_left_grid());
    grid.extend(body_back_grid());
    grid.extend(body_right_grid());
    grid.extend(body_top_grid());
    grid.extend(body_bottom_grid());

    grid
}