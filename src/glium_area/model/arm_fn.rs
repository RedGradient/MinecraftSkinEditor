use crate::glium_area::model::{CELL_SIZE, generate_indexes, GRID_COLOR};
use crate::glium_area::vertex::Vertex;

const ARM_CELLS_COUNT: usize = 224;

struct CellsAndLines {
    cells: Vec<Vertex>,
    grid: Vec<Vertex>,
}

fn arm_front() -> Vec<Vertex> {
    let width = 4;
    let height = 12;

    let mut vertices = Vec::with_capacity(width * height * 4);

    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        let z = 0.25;
        for j in 0..width {
            let x = -0.25 + j as f32 * CELL_SIZE;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: [i as f32 / height as f32, j as f32 / height as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x + CELL_SIZE, y, z],
                color: [i as f32 / height as f32, j as f32 / height as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x + CELL_SIZE, y - CELL_SIZE, z],
                color: [i as f32 / height as f32, j as f32 / height as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y - CELL_SIZE, z],
                color: [i as f32 / height as f32, j as f32 / height as f32, 0.0, 1.0]});
        }
    }

    vertices
}
fn arm_left() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 4;
    let height = 12;

    let grid_size = 8;

    for i in 0..height {
        for j in 0..width {
            let x = 0.25;
            let y = 0.75 - i as f32 * CELL_SIZE;
            let z = 0.25 - j as f32 * CELL_SIZE;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y, z - CELL_SIZE],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y - CELL_SIZE, z - CELL_SIZE],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y - CELL_SIZE, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
        }
    }

    vertices
}
fn arm_back() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 4;
    let height = 12;

    let grid_size = 8;

    for i in 0..height {
        for j in 0..width {
            let x = 0.25 - j as f32 * CELL_SIZE;
            let y = 0.75 - i as f32 * CELL_SIZE;
            let z = -0.25;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x - CELL_SIZE, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x - CELL_SIZE, y - CELL_SIZE, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y - CELL_SIZE, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
        }
    }

    vertices
}
fn arm_right() -> Vec<Vertex> {
    let width = 4;
    let height = 12;

    let mut vertices = Vec::with_capacity(width * height * 4);
    
    for i in 0..height {
        for j in 0..width {
            let x = -0.25;
            let y = 0.75 - (i as f32) * CELL_SIZE;
            let z = -0.25 + (j as f32) * CELL_SIZE;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: [i as f32 / height as f32, j as f32 / width as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y, z + CELL_SIZE],
                color: [i as f32 / height as f32, j as f32 / width as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y - CELL_SIZE, z + CELL_SIZE],
                color: [i as f32 / height as f32, j as f32 / width as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y - CELL_SIZE, z],
                color: [i as f32 / height as f32, j as f32 / width as f32, 0.0, 1.0]});
        }
    }

    vertices
}
fn arm_top() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 4;
    let height = 4;

    let grid_size = 8;

    for i in 0..height {
        for j in 0..width {
            let x = -0.25 + j as f32 * CELL_SIZE;
            let y = 0.75;
            let z = -0.25 + i as f32 * CELL_SIZE;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x + CELL_SIZE, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x + CELL_SIZE, y, z + CELL_SIZE],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y, z + CELL_SIZE],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
        }
    }

    vertices
}
fn arm_bottom() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 4;
    let height = 4;

    let grid_size = 8;

    for i in 0..height {
        for j in 0..width {
            let x = -0.25 + j as f32 * CELL_SIZE;
            let y = -0.75;
            let z = -0.25 + i as f32 * CELL_SIZE;

            // --- 4 VERTEXES ---
            // vertices.push(Vertex2 {
            //     position: [x, y, z],
            //     color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            // vertices.push(Vertex2 {
            //     position: [x + CELL_SIZE, y, z],
            //     color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            // vertices.push(Vertex2 {
            //     position: [x + CELL_SIZE, y, z + CELL_SIZE],
            //     color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            // vertices.push(Vertex2 {
            //     position: [x, y, z + CELL_SIZE],
            //     color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});

            vertices.push(Vertex {
                position: [x, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y, z + CELL_SIZE],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x + CELL_SIZE, y, z + CELL_SIZE],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x + CELL_SIZE, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
        }
    }

    vertices
}


pub fn arm_vertexes() -> Vec<Vertex> {
    let mut vertexes = Vec::with_capacity(ARM_CELLS_COUNT);

    vertexes.extend(arm_front());
    vertexes.extend(arm_left());
    vertexes.extend(arm_back());
    vertexes.extend(arm_right());
    vertexes.extend(arm_top());
    vertexes.extend(arm_bottom());

    vertexes
}

pub fn arm_indexes() -> Vec<u16> {
    generate_indexes(ARM_CELLS_COUNT)
}

// ------------
// --- GRID ---
// ------------
fn arm_front_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width: u32 = 4;
    let height: u32 = 12;

    let z = 0.25;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..=width {
            let x = -0.25 + j as f32 * CELL_SIZE;

            // --- VERTICAL LINE ---
            grid.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            grid.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }

        // --- HORIZONTAL LINE ---
        grid.push(Vertex { position: [-0.25, y, z], color: GRID_COLOR});
        grid.push(Vertex { position: [0.25, y, z], color: GRID_COLOR});
    }

    grid
}
fn arm_left_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 4;
    let height = 12;
    
    let x = 0.25;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let z = 0.25 - j as f32 * CELL_SIZE;

            // --- VERTICAL LINE ---
            grid.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            grid.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        grid.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
        grid.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
    }

    grid
}
fn arm_back_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 4;
    let height = 12;
    
    let z = -0.25;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = 0.25 - j as f32 * CELL_SIZE;

            // --- VERTICAL LINE ---
            grid.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            grid.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }

        // --- HORIZONTAL LINE ---
        grid.push(Vertex { position: [-0.25, y, z], color: GRID_COLOR});
        grid.push(Vertex { position: [0.25, y, z], color: GRID_COLOR});
    }

    grid
}
fn arm_right_grid() -> Vec<Vertex> {
    let width = 4;
    let height = 12;

    let mut grid = Vec::with_capacity(width * height * 4);

    let x = -0.25;
    for i in 0..height {
        let y = 0.75 - (i as f32) * CELL_SIZE;
        for j in 0..width {
            let z = -0.25 + (j as f32) * CELL_SIZE;
            // --- VERTICAL LINE ---
            grid.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            grid.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        grid.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
        grid.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
    }

    grid
}
fn arm_top_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 4;
    let height = 4;

    let y = 0.75;
    for i in 0..height {
        let z = -0.25 + i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = -0.25 + j as f32 * CELL_SIZE;

            grid.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
            grid.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
        }
        grid.push(Vertex { position: [-0.25, y, z], color: GRID_COLOR});
        grid.push(Vertex { position: [0.25, y, z], color: GRID_COLOR});
    }

    grid
}
fn arm_bottom_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 4;
    let height = 4;
    
    let y = -0.75;
    for i in 0..height {
        let z = -0.25 + i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = -0.25 + j as f32 * CELL_SIZE;

            grid.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
            grid.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
        }
        grid.push(Vertex { position: [-0.25, y, z], color: GRID_COLOR});
        grid.push(Vertex { position: [0.25, y, z], color: GRID_COLOR});
    }

    grid
}

pub fn arm_grid() -> Vec<Vertex> {
    let mut grid = Vec::new();

    grid.extend(arm_front_grid());
    grid.extend(arm_left_grid());
    grid.extend(arm_back_grid());
    grid.extend(arm_right_grid());
    grid.extend(arm_top_grid());
    grid.extend(arm_bottom_grid());

    grid
}