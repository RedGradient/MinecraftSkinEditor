use nalgebra_glm::Vec2;

#[derive(Clone, Copy)]
pub struct MouseMove {
    prev: Vec2,
    curr: Vec2,
}

impl MouseMove {
    pub fn new(curr_x: f32, curr_y: f32) -> Self {
        MouseMove {
            prev: Vec2::new(curr_x, curr_y),
            curr: Vec2::new(curr_x, curr_y),
        }
    }

    pub fn get_delta(&self) -> Vec2 {
        Vec2::new(
            self.curr.x - self.prev.x,
            self.curr.y - self.prev.y,
        )
    }

    pub fn move_to(&mut self, new_x: f32, new_y: f32) {
        self.prev = self.curr;
        self.curr = Vec2::new(new_x, new_y);
    }
}
