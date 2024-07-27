use crate::glium_area::model::{CELL_COLOR, CELL_SIZE, GRID_COLOR};
use crate::glium_area::vertex::Vertex;

const ARM_CLASSIC_CELLS_COUNT: usize = 224;
const ARM_SLIM_CELLS_COUNT: usize = 192;

// ---------------------
// --- CLASSIC MODEL ---
fn front(vertices: &mut Vec<Vertex>) {
    let width = 4;
    let height = 12;
    
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        let z = 0.25;
        for j in 0..width {
            let x = -0.25 + j as f32 * CELL_SIZE;

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
            let x = 0.25;
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
    let width = 4;
    let height = 12;
    
    for i in 0..height {
        for j in 0..width {
            let x = 0.25 - j as f32 * CELL_SIZE;
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
            let x = -0.25;
            let y = 0.75 - (i as f32) * CELL_SIZE;
            let z = -0.25 + (j as f32) * CELL_SIZE;

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
    let width = 4;
    let height = 4;
    
    for i in 0..height {
        for j in 0..width {
            let x = -0.25 + j as f32 * CELL_SIZE;
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
    let width = 4;
    let height = 4;

    for i in 0..height {
        for j in 0..width {
            let x = -0.25 + j as f32 * CELL_SIZE;
            let y = -0.75;
            let z = -0.25 + i as f32 * CELL_SIZE;

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

pub fn cuboid_4x12x4() -> Vec<Vertex> {
    let mut vertexes = Vec::with_capacity(ARM_CLASSIC_CELLS_COUNT * 4);
    
    front(&mut vertexes);
    left(&mut vertexes);
    back(&mut vertexes);
    right(&mut vertexes);
    top(&mut vertexes);
    bottom(&mut vertexes);
    
    vertexes
}


// ------------------
// --- SLIM MODEL ---
fn front_s(vertices: &mut Vec<Vertex>) {
    let width = 3;
    let height = 12;
    
    for i in 0..height {
        for j in 0..width {
            let x = -0.1875 + j as f32 * CELL_SIZE;
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
fn left_s(vertices: &mut Vec<Vertex>) {
    let width = 4;
    let height = 12;
    
    for i in 0..height {
        for j in 0..width {
            let x = 0.1875;
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
fn back_s(vertices: &mut Vec<Vertex>) {
    let width = 3;
    let height = 12;
    
    for i in 0..height {
        for j in 0..width {
            let x = 0.1875 - j as f32 * CELL_SIZE;
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
fn right_s(vertices: &mut Vec<Vertex>) {
    let width = 4;
    let height = 12;
    
    for i in 0..height {
        for j in 0..width {
            let x = -0.1875;
            let y = 0.75 - (i as f32) * CELL_SIZE;
            let z = -0.25 + (j as f32) * CELL_SIZE;

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
fn top_s(vertices: &mut Vec<Vertex>) {
    let width = 3;
    let height = 4;
    
    for i in 0..height {
        for j in 0..width {
            let x = -0.1875 + j as f32 * CELL_SIZE;
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
fn bottom_s(vertices: &mut Vec<Vertex>) {
    let width = 3;
    let height = 4;
    
    for i in 0..height {
        for j in 0..width {
            let x = -0.1875 + j as f32 * CELL_SIZE;
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

pub fn cuboid_3x12x4() -> Vec<Vertex> {
    let mut vertices = Vec::with_capacity(ARM_SLIM_CELLS_COUNT * 4);

    front_s(&mut vertices);
    left_s(&mut vertices);
    back_s(&mut vertices);
    right_s(&mut vertices);
    top_s(&mut vertices);
    bottom_s(&mut vertices);

    vertices
}


// --------------------
// --- GRID CLASSIC ---
fn front_grid(vertices: &mut Vec<Vertex>) {
    let width: u32 = 4;
    let height: u32 = 12;

    let z = 0.25;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = -0.25 + j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [-0.25, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.25, y, z], color: GRID_COLOR});
    }
}
fn left_grid(vertices: &mut Vec<Vertex>) {
    let width = 4;
    let height = 12;
    
    let x = 0.25;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let z = 0.25 - j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
        vertices.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
    }
}
fn back_grid(vertices: &mut Vec<Vertex>) {
    let width = 4;
    let height = 12;
    
    let z = -0.25;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = 0.25 - j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [-0.25, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.25, y, z], color: GRID_COLOR});
    }
}
fn right_grid(vertices: &mut Vec<Vertex>) {
    let width = 4;
    let height = 12;

    let x = -0.25;
    for i in 0..height {
        let y = 0.75 - (i as f32) * CELL_SIZE;
        for j in 0..width {
            let z = -0.25 + (j as f32) * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
        vertices.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
    }
}
fn top_grid(vertices: &mut Vec<Vertex>) {
    let width = 4 + 1;
    let height = 4 + 1;

    let y = 0.75;
    for i in 0..height {
        let z = -0.25 + i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = -0.25 + j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [-0.25, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.25, y, z], color: GRID_COLOR});
    }
}
fn bottom_grid(vertices: &mut Vec<Vertex>) {
    let width = 4 + 1;
    let height = 4 + 1;
    
    let y = -0.75;
    for i in 0..height {
        let z = -0.25 + i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = -0.25 + j as f32 * CELL_SIZE;

            vertices.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
        }
        vertices.push(Vertex { position: [-0.25, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.25, y, z], color: GRID_COLOR});
    }
}

pub fn grid_4x12x4() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    front_grid(&mut vertices);
    left_grid(&mut vertices);
    back_grid(&mut vertices);
    right_grid(&mut vertices);
    top_grid(&mut vertices);
    bottom_grid(&mut vertices);

    vertices
}


// --------------------
// --- GRID SLIM ---
fn front_grid_s(vertices: &mut Vec<Vertex>) {
    let width_lines_count = 3 + 1;
    let height_lines_count = 12 + 1;
    
    let z = 0.25;
    for i in 0..height_lines_count {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width_lines_count {
            let x = -0.1875 + j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [-0.1875, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.1875, y, z], color: GRID_COLOR});
    }
}
fn left_grid_s(vertices: &mut Vec<Vertex>) {
    let width_lines_count = 4 + 1;
    let height_lines_count = 12 + 1;
    
    let x = 0.1875;
    for i in 0..height_lines_count {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width_lines_count {
            let z = 0.25 - j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
        vertices.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
    }
}
fn back_grid_s(vertices: &mut Vec<Vertex>) {
    let width_lines_count = 3 + 1;
    let height_lines_count = 12 + 1;
    
    let z = -0.25;
    for i in 0..height_lines_count {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width_lines_count {
            let x = 0.1875 - j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [-0.1875, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.1875, y, z], color: GRID_COLOR});
    }
}
fn right_grid_s(vertices: &mut Vec<Vertex>) {
    let width_lines_count = 4 + 1;
    let height_lines_count = 12 + 1;
    
    let x = -0.1875;
    for i in 0..height_lines_count {
        let y = 0.75 - (i as f32) * CELL_SIZE;
        for j in 0..width_lines_count {
            let z = -0.25 + (j as f32) * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, -0.75, z], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, 0.75, z], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
        vertices.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
    }
}
fn top_grid_s(vertices: &mut Vec<Vertex>) {
    let width_lines_count = 3 + 1;
    let height_lines_count = 4 + 1;
    
    let y = 0.75;
    for i in 0..height_lines_count {
        let z = -0.25 + i as f32 * CELL_SIZE;
        for j in 0..width_lines_count {
            let x = -0.1875 + j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [-0.1875, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.1875, y, z], color: GRID_COLOR});
    }
}
fn bottom_grid_s(vertices: &mut Vec<Vertex>) {
    let width_lines_count = 3 + 1;
    let height_lines_count = 4 + 1;

    let y = -0.75;
    for i in 0..height_lines_count {
        let z = -0.25 + i as f32 * CELL_SIZE;
        for j in 0..width_lines_count {
            let x = -0.1875 + j as f32 * CELL_SIZE;
            // --- VERTICAL LINE ---
            vertices.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
            vertices.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
        }
        // --- HORIZONTAL LINE ---
        vertices.push(Vertex { position: [-0.1875, y, z], color: GRID_COLOR});
        vertices.push(Vertex { position: [0.1875, y, z], color: GRID_COLOR});
    }
}

pub fn grid_3x12x4() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    front_grid_s(&mut vertices);
    left_grid_s(&mut vertices);
    back_grid_s(&mut vertices);
    right_grid_s(&mut vertices);
    top_grid_s(&mut vertices);
    bottom_grid_s(&mut vertices);

    vertices
}