use crate::glium_area::model::{CELL_SIZE, generate_indexes, GRID_COLOR};
use crate::glium_area::vertex::Vertex;

const CELLS_COUNT_CLASSIC: usize = 224;
const CELLS_COUNT_SLIM: usize = 192;

// ---------------------
// --- CLASSIC MODEL ---
fn front() -> Vec<Vertex> {
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
fn left() -> Vec<Vertex> {
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
fn back() -> Vec<Vertex> {
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
fn right() -> Vec<Vertex> {
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
fn top() -> Vec<Vertex> {
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
fn bottom() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 4;
    let height = 4;

    let grid_size = 8;

    for i in 0..height {
        for j in 0..width {
            let x = -0.25 + j as f32 * CELL_SIZE;
            let y = -0.75;
            let z = -0.25 + i as f32 * CELL_SIZE;

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
pub fn cuboid_4x12x4() -> Vec<Vertex> {
    let mut vertexes = Vec::with_capacity(CELLS_COUNT_CLASSIC);

    vertexes.extend(front());
    vertexes.extend(left());
    vertexes.extend(back());
    vertexes.extend(right());
    vertexes.extend(top());
    vertexes.extend(bottom());

    vertexes
}


// ------------------
// --- SLIM MODEL ---
fn front_s() -> Vec<Vertex> {
    let width = 3;
    let height = 12;

    let mut vertices = Vec::with_capacity(width * height);

    for i in 0..height {
        for j in 0..width {
            let x = -0.1875 + j as f32 * CELL_SIZE;
            let y = 0.75 - i as f32 * CELL_SIZE;
            let z = 0.25;

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
fn left_s() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 4;
    let height = 12;

    let grid_size = 8;

    for i in 0..height {
        for j in 0..width {
            let x = 0.1875;
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
fn back_s() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 3;
    let height = 12;

    let grid_size = 8;

    for i in 0..height {
        for j in 0..width {
            let x = 0.1875 - j as f32 * CELL_SIZE;
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
fn right_s() -> Vec<Vertex> {
    let width = 4;
    let height = 12;

    let mut vertices = Vec::with_capacity(width * height * 4);

    for i in 0..height {
        for j in 0..width {
            let x = -0.1875;
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
fn top_s() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 3;
    let height = 4;

    let grid_size = 8;

    for i in 0..height {
        for j in 0..width {
            let x = -0.1875 + j as f32 * CELL_SIZE;
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
fn bottom_s() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 3;
    let height = 4;

    let grid_size = 8;

    for i in 0..height {
        for j in 0..width {
            let x = -0.1875 + j as f32 * CELL_SIZE;
            let y = -0.75;
            let z = -0.25 + i as f32 * CELL_SIZE;

            // --- 4 VERTEXES ---
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
pub fn cuboid_3x12x4() -> Vec<Vertex> {
    let mut vertexes = Vec::with_capacity(CELLS_COUNT_SLIM);

    vertexes.extend(front_s());
    vertexes.extend(left_s());
    vertexes.extend(back_s());
    vertexes.extend(right_s());
    vertexes.extend(top_s());
    vertexes.extend(bottom_s());

    vertexes
}


// ---------------
// --- INDEXES ---
pub fn arm_indexes() -> Vec<u16> {
    generate_indexes(CELLS_COUNT_CLASSIC)
}

// --------------------
// --- GRID CLASSIC ---
fn front_grid() -> Vec<Vertex> {
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
fn left_grid() -> Vec<Vertex> {
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
fn back_grid() -> Vec<Vertex> {
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
fn right_grid() -> Vec<Vertex> {
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
fn top_grid() -> Vec<Vertex> {
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
fn bottom_grid() -> Vec<Vertex> {
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
pub fn grid_4x12x4() -> Vec<Vertex> {
    let mut grid = Vec::new();

    grid.extend(front_grid());
    grid.extend(left_grid());
    grid.extend(back_grid());
    grid.extend(right_grid());
    grid.extend(top_grid());
    grid.extend(bottom_grid());

    grid
}


// --------------------
// --- GRID SLIM ---
fn front_grid_s() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width: u32 = 3;
    let height: u32 = 12;

    let z = 0.25;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..=width {
            let x = -0.1875 + j as f32 * CELL_SIZE;

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
fn left_grid_s() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 4;
    let height = 12;

    let x = 0.1875;
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
fn back_grid_s() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 3;
    let height = 12;

    let z = -0.25;
    for i in 0..height {
        let y = 0.75 - i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = 0.1875 - j as f32 * CELL_SIZE;

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
fn right_grid_s() -> Vec<Vertex> {
    let width = 4;
    let height = 12;

    let mut grid = Vec::with_capacity(width * height * 4);

    let x = -0.1875;
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
fn top_grid_s() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 3;
    let height = 4;

    let y = 0.75;
    for i in 0..height {
        let z = -0.25 + i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = -0.1875 + j as f32 * CELL_SIZE;

            grid.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
            grid.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
        }
        grid.push(Vertex { position: [-0.25, y, z], color: GRID_COLOR});
        grid.push(Vertex { position: [0.25, y, z], color: GRID_COLOR});
    }

    grid
}
fn bottom_grid_s() -> Vec<Vertex> {
    let mut grid = Vec::new();

    let width = 3;
    let height = 4;

    let y = -0.75;
    for i in 0..height {
        let z = -0.25 + i as f32 * CELL_SIZE;
        for j in 0..width {
            let x = -0.1875 + j as f32 * CELL_SIZE;

            grid.push(Vertex { position: [x, y, -0.25], color: GRID_COLOR});
            grid.push(Vertex { position: [x, y, 0.25], color: GRID_COLOR});
        }
        grid.push(Vertex { position: [-0.25, y, z], color: GRID_COLOR});
        grid.push(Vertex { position: [0.25, y, z], color: GRID_COLOR});
    }

    grid
}
pub fn grid_3x12x4() -> Vec<Vertex> {
    let mut grid = Vec::new();

    grid.extend(front_grid_s());
    grid.extend(left_grid_s());
    grid.extend(back_grid_s());
    grid.extend(right_grid_s());
    grid.extend(top_grid_s());
    grid.extend(bottom_grid_s());

    grid
}