use std::hash::Hash;

use CubeSide::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub enum CubeSide {
    Front,
    Left,
    Back,
    Right,
    Top,
    Bottom,
}