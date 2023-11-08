pub mod body_fn;
pub mod arm_fn;
pub mod head_fn;

const CELL_SIZE: f32 = 1.0 / 8.0;
const LINE_COLOR: [f32; 4] = [0., 0., 0., 1.0];

pub fn generate_indexes(cells_count: usize) -> Vec<u16> {
    let mut indices = Vec::with_capacity(cells_count * 6);

    for i in 0..cells_count {
        let base_index = i * 4;
        indices.push(base_index as u16);
        indices.push((base_index + 1) as u16);
        indices.push((base_index + 2) as u16);
        indices.push((base_index + 2) as u16);
        indices.push((base_index + 3) as u16);
        indices.push(base_index as u16);
    }
    indices
}