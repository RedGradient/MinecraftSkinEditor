use glm::{Mat4, Vec2};
use nalgebra_glm as glm;

pub struct Camera {
    pub position: glm::Vec3,
    pub yaw: f32,
    pub pitch: f32,
    rotation_matrix: Mat4,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: glm::TVec3::new(0.0, 0.0, 3.0),
            yaw: 0.0,
            pitch: 0.0,
            rotation_matrix: Mat4::identity(),
        }
    }

    pub fn get_rotation_matrix(&self) -> Mat4 {
        self.rotation_matrix
    }

    pub fn update_yaw_and_pitch(&mut self, mouse_delta: Vec2) {
        const SENSITIVITY: f32 = 0.5;

        // set the rotation angle limit; after 360 the counting starts from zero
        self.yaw = (self.yaw + mouse_delta.x * SENSITIVITY) % 360.0;
        self.pitch += mouse_delta.y * SENSITIVITY;

        // limit the pitch to avoid camera rollover
        self.pitch = self.pitch.clamp(-90.0, 90.0);

        // update rotation matrix
        self.rotation_matrix = glm::rotate_x(&Mat4::identity(), self.pitch.to_radians());
        self.rotation_matrix = glm::rotate_y(&self.rotation_matrix, self.yaw.to_radians());
    }

    pub fn update_distance(&mut self, distance: f32) {
        self.position.z = self.position.z - distance;
    }

    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at_rh(
            &self.position,
            &glm::TVec3::new(0.0, 0.0, 0.0),
            &glm::vec3(0.0, 1.0, 0.0)
        )
    }
}