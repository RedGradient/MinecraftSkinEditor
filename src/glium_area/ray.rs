use nalgebra_glm as glm;

pub struct Ray {
    pub origin: glm::Vec3,
    pub direction: glm::Vec3,
}

impl Ray {
    pub fn new(origin: glm::Vec3, direction: glm::Vec3) -> Self {
        Ray { origin, direction }
    }
}
