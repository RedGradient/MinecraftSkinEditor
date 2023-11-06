#[derive(Clone, Copy)]
pub struct CrossInfo {
    pub cell_index: usize,
    pub dist: f32,
}

impl PartialEq for CrossInfo {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl PartialOrd for CrossInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}