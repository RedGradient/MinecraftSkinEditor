pub mod meshes;
mod obj_loader;

pub use meshes::{
    body_grid, body_vertices, cuboid_3x12x4, cuboid_4x12x4, grid_3x12x4, grid_4x12x4, head_grid,
    head_vertices,
};

#[cfg(test)]
mod export_obj;

pub fn generate_indexes(cells_count: usize) -> Vec<u16> {
    let mut indices = Vec::with_capacity(cells_count * 6);

    for i in 0..cells_count {
        let base_index = i * 4;

        // clockwise
        indices.push((base_index + 1) as u16);
        indices.push(base_index as u16);
        indices.push((base_index + 2) as u16);
        indices.push((base_index + 2) as u16);
        indices.push(base_index as u16);
        indices.push((base_index + 3) as u16);
    }
    indices
}
