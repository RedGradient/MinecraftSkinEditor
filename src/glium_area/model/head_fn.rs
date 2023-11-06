use crate::glium_area::model::generate_indexes;
use crate::glium_area::vertex::Vertex;

const HEAD_CELLS_COUNT: usize = 384;

fn head_front() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 8;
    let height = 8;

    let cell_size = 1.0 / 8.0;
    let grid_size = 8;

    for i in 0..height {
        for j in 0..width {
            let x = -0.5 + j as f32 * cell_size;
            let y = 0.5 - i as f32 * cell_size;
            let z = 0.5;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x + cell_size, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x + cell_size, y - cell_size, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y - cell_size, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
        }
    }

    vertices
}
fn head_back() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 8;
    let height = 8;

    let cell_size = 1.0 / 8.0;
    let grid_size = 8;

    for i in 0..height {
        for j in 0..width {
            let x = 0.5 - j as f32 * cell_size;
            let y = 0.5 - i as f32 * cell_size;
            let z = -0.5;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x - cell_size, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x - cell_size, y - cell_size, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y - cell_size, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
        }
    }

    vertices
}
fn head_right() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let grid_size = 8;
    let cell_size = 1.0 / grid_size as f32;

    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = -0.5;
            let y = 0.5 - i as f32 * cell_size;
            let z = -0.5 + j as f32 * cell_size;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y, z + cell_size],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y - cell_size, z + cell_size],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y - cell_size, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
        }
    }

    vertices
}
fn head_left() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let grid_size = 8;
    let cell_size = 1.0 / grid_size as f32;

    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = 0.5;
            let y = 0.5 - i as f32 * cell_size;
            let z = 0.5 - j as f32 * cell_size;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y, z - cell_size],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y - cell_size, z - cell_size],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y - cell_size, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
        }
    }

    vertices
}
fn head_top() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let grid_size = 8;
    let cell_size = 1.0 / grid_size as f32;

    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = -0.5 + j as f32 * cell_size;
            let y = 0.5;
            let z = -0.5 + i as f32 * cell_size;

            // --- 4 VERTEXES ---
            vertices.push(Vertex {
                position: [x, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x + cell_size, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x + cell_size, y, z + cell_size],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y, z + cell_size],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
        }
    }

    vertices
}
fn head_bottom() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let grid_size = 8;
    let cell_size = 1.0 / grid_size as f32;

    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = -0.5 + j as f32 * cell_size;
            let y = -0.5;
            let z = -0.5 + i as f32 * cell_size;

            // --- 4 VERTEXES ---
            // vertices.push(Vertex2 {
            //     position: [x, y, z],
            //     color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            // vertices.push(Vertex2 {
            //     position: [x + cell_size, y, z],
            //     color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            // vertices.push(Vertex2 {
            //     position: [x + cell_size, y, z + cell_size],
            //     color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            // vertices.push(Vertex2 {
            //     position: [x, y, z + cell_size],
            //     color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});

            vertices.push(Vertex {
                position: [x, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x, y, z + cell_size],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x + cell_size, y, z + cell_size],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
            vertices.push(Vertex {
                position: [x + cell_size, y, z],
                color: [i as f32 / grid_size as f32, j as f32 / grid_size as f32, 0.0, 1.0]});
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
