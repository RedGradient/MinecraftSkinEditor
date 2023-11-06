use crate::glium_area::model::generate_indexes;
use crate::glium_area::vertex::Vertex;

const BODY_CELLS_COUNT: usize = 352;

fn body_front() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 8;
    let height = 12;

    let cell_size = 1.0 / 8.0;
    let grid_size = 8;

    for i in 0..height {
        for j in 0..width {
            let x = -0.5 + j as f32 * cell_size;
            let y = 0.75 - i as f32 * cell_size;
            let z = 0.25;

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

fn body_left() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 4;
    let height = 12;

    let cell_size = 1.0 / 8.0;
    let grid_size = 8;

    for i in 0..height {
        for j in 0..width {
            let x = 0.5;
            let y = 0.75 - i as f32 * cell_size;
            let z = 0.25 - j as f32 * cell_size;

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

fn body_back() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 8;
    let height = 12;

    let cell_size = 1.0 / 8.0;
    let grid_size = 8;

    for i in 0..height {
        for j in 0..width {
            let x = 0.5 - j as f32 * cell_size;
            let y = 0.75 - i as f32 * cell_size;
            let z = -0.25;

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

fn body_right() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 4;
    let height = 12;

    let grid_size = 8;
    let cell_size = 1.0 / grid_size as f32;

    for i in 0..height {
        for j in 0..width {
            let x = -0.5;
            let y = 0.75 - i as f32 * cell_size;
            let z = -0.25 + j as f32 * cell_size;

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

fn body_top() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 8;
    let height = 4;

    let grid_size = 8;
    let cell_size = 1.0 / grid_size as f32;

    for i in 0..height {
        for j in 0..width {
            let x = -0.5 + j as f32 * cell_size;
            let y = 0.75;
            let z = -0.25 + i as f32 * cell_size;

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

fn body_bottom() -> Vec<Vertex> {
    let mut vertices = Vec::new();

    let width = 8;
    let height = 4;

    let grid_size = 8;
    let cell_size = 1.0 / grid_size as f32;

    for i in 0..height {
        for j in 0..width {
            let x = -0.5 + j as f32 * cell_size;
            let y = -0.75;
            let z = -0.25 + i as f32 * cell_size;

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