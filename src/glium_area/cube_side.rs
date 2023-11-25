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


pub const SIDES: [CubeSide; 6] = [
    CubeSide::Front,
    CubeSide::Left,
    CubeSide::Back,
    CubeSide::Right,
    CubeSide::Top,
    CubeSide::Bottom,
];

impl TryFrom<usize> for CubeSide {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Front),
            1 => Ok(Left),
            2 => Ok(Back),
            3 => Ok(Right),
            4 => Ok(Top),
            5 => Ok(Bottom),
            _ => Err(())
        }
    }
}