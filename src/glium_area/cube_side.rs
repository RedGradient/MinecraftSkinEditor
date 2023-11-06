use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub enum CubeSide {
    Front,
    Left,
    Back,
    Right,
    Top,
    Bottom,

    // Left,
    // Right,
    // Top,
    // Bottom,
    // Back,
    // Front,
}


pub const SIDES: [CubeSide; 6] = [
    CubeSide::Front,
    CubeSide::Left,
    CubeSide::Back,
    CubeSide::Right,
    CubeSide::Top,
    CubeSide::Bottom,
];
