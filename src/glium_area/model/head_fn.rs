use crate::glium_area::model::{CELL_SIZE, generate_indexes, GRID_COLOR, CELL_COLOR};
use crate::glium_area::vertex::Vertex;


const GRID_SIZE: usize = 8;
const HEAD_CELLS_COUNT: usize = 384;

fn head_front() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 8;
    let height = 8;

    for i in 0..height {
        for j in 0..width {
            let x = -0.5 + j as f32 * CELL_SIZE;
            let y = 0.5 - i as f32 * CELL_SIZE;
            let z = 0.5;

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
fn head_back() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = 0.5 - j as f32 * CELL_SIZE;
            let y = 0.5 - i as f32 * CELL_SIZE;
            let z = -0.5;

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
fn head_right() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let x = -0.5;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let y = 0.5 - i as f32 * CELL_SIZE;
            let z = -0.5 + j as f32 * CELL_SIZE;

            // --- 4 VERTEXES ---
            grid.push(Vertex {
                position: [x, y, z],
                color: CELL_COLOR});
            grid.push(Vertex {
                position: [x, y, z + CELL_SIZE],
                color: CELL_COLOR});
            grid.push(Vertex {
                position: [x, y - CELL_SIZE, z + CELL_SIZE],
                color: CELL_COLOR});
            grid.push(Vertex {
                position: [x, y - CELL_SIZE, z],
                color: CELL_COLOR});
        }
    }

    grid
}
fn head_left() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = 0.5;
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

    vertices
}
fn head_top() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = -0.5 + j as f32 * CELL_SIZE;
            let y = 0.5;
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

    vertices
}
fn head_bottom() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = -0.5 + j as f32 * CELL_SIZE;
            let y = -0.5;
            let z = -0.5 + i as f32 * CELL_SIZE;

            // --- 4 VERTEXES ---
            // vertices.push(Vertex2 {
            //     position: [x, y, z],
            //     color: CELL_COLOR});
            // vertices.push(Vertex2 {
            //     position: [x + CELL_SIZE, y, z],
            //     color: CELL_COLOR});
            // vertices.push(Vertex2 {
            //     position: [x + CELL_SIZE, y, z + CELL_SIZE],
            //     color: CELL_COLOR});
            // vertices.push(Vertex2 {
            //     position: [x, y, z + CELL_SIZE],
            //     color: CELL_COLOR});

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


pub fn head_vertexes() -> Vec<Vertex> {
    let mut vertexes = Vec::with_capacity(HEAD_CELLS_COUNT);

    vertexes.extend(head_front());
    vertexes.extend(head_left());
    vertexes.extend(head_back());
    vertexes.extend(head_right());
    vertexes.extend(head_top());
    vertexes.extend(head_bottom());

    vertexes
}

pub fn head_indexes() -> Vec<u16> {
    generate_indexes(HEAD_CELLS_COUNT)
}


// ------------
// --- GRID ---
// ------------
fn head_front_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let z = 0.5;
    for i in 0..GRID_SIZE {
        let y = 0.5 - i as f32 * CELL_SIZE;
        for j in 0..GRID_SIZE {
            let x = -0.5 + j as f32 * CELL_SIZE;
            // --- 2 VERTICAL LINES ---
            grid.push(Vertex { position: [x, -0.5, z], color: GRID_COLOR});
            grid.push(Vertex { position: [x, 0.5, z], color: GRID_COLOR});
        }

        // --- 2 HORIZONTAL LINES ---
        grid.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        grid.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }

    grid
}
fn head_left_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let x = 0.5;
    for i in 0..GRID_SIZE {
        let y = 0.5 - i as f32 * CELL_SIZE;
        for j in 0..GRID_SIZE {
            let z = 0.5 - j as f32 * CELL_SIZE;
            // --- 2 VERTICAL LINES ---
            grid.push(Vertex { position: [x, -0.5, z], color: GRID_COLOR});
            grid.push(Vertex { position: [x, 0.5, z], color: GRID_COLOR});
        }
        // --- 2 HORIZONTAL LINES ---
        grid.push(Vertex { position: [x, y, -0.5], color: GRID_COLOR});
        grid.push(Vertex { position: [x, y, 0.5], color: GRID_COLOR});
    }

    grid
}
fn head_back_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 8;
    let height = 8;

    let z = -0.5;
    for i in 0..height {
        let y = 0.5 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = 0.5 - j as f32 * CELL_SIZE;
            // --- 2 VERTICAL LINES ---
            grid.push(Vertex { position: [x, -0.5, z], color: GRID_COLOR});
            grid.push(Vertex { position: [x, 0.5, z], color: GRID_COLOR});
        }

        // --- 2 HORIZONTAL LINES ---
        grid.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        grid.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }

    grid
}
fn head_right_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let x = -0.5;
    for i in 0..GRID_SIZE {
        let y = 0.5 - i as f32 * CELL_SIZE;
        for j in 0..GRID_SIZE {
            let z = -0.5 + j as f32 * CELL_SIZE;
            // --- 2 VERTICAL LINES ---
            grid.push(Vertex { position: [x, -0.5, z], color: GRID_COLOR});
            grid.push(Vertex { position: [x, 0.5, z], color: GRID_COLOR});
        }
        // --- 2 HORIZONTAL LINES ---
        grid.push(Vertex { position: [x, y, -0.5], color: GRID_COLOR});
        grid.push(Vertex { position: [x, y, 0.5], color: GRID_COLOR});
    }

    grid
}
fn head_top_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let y = 0.5;
    for i in 0..GRID_SIZE {
        let z = -0.5 + i as f32 * CELL_SIZE;
        for j in 0..GRID_SIZE {
            let x = -0.5 + j as f32 * CELL_SIZE;
            grid.push(Vertex { position: [x, y, -0.5], color: GRID_COLOR});
            grid.push(Vertex { position: [x, y, 0.5], color: GRID_COLOR});
        }
        grid.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        grid.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }

    grid
}
fn head_bottom_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let y = -0.5;
    for i in 0..GRID_SIZE {
        let z = -0.5 + i as f32 * CELL_SIZE;
        for j in 0..GRID_SIZE {
            let x = -0.5 + j as f32 * CELL_SIZE;
            grid.push(Vertex { position: [x, y, -0.5], color: GRID_COLOR});
            grid.push(Vertex { position: [x, y, 0.5], color: GRID_COLOR});
        }
        grid.push(Vertex { position: [-0.5, y, z], color: GRID_COLOR});
        grid.push(Vertex { position: [0.5, y, z], color: GRID_COLOR});
    }

    grid
}


pub fn head_grid() -> Vec<Vertex> {
    let mut grid = Vec::with_capacity(HEAD_CELLS_COUNT);

    grid.extend(head_front_grid());
    grid.extend(head_left_grid());
    grid.extend(head_back_grid());
    grid.extend(head_right_grid());
    grid.extend(head_top_grid());
    grid.extend(head_bottom_grid());

    grid
}