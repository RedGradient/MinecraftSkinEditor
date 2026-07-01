use nalgebra_glm as glm;

use crate::glium_area::ray::Ray;

const EPS: f32 = 1e-5;

/// Face order matches mesh/OBJ layout: front, left, back, right, top, bottom.
pub fn ray_local_aabb(
    origin: glm::Vec3,
    direction: glm::Vec3,
    bounds_min: glm::Vec3,
    bounds_max: glm::Vec3,
) -> Option<(f32, usize)> {
    let mut t_min = f32::NEG_INFINITY;
    let mut t_max = f32::INFINITY;
    let mut enter_axis = 0usize;
    let mut enter_on_max = false;

    for axis in 0..3 {
        if direction[axis].abs() < EPS {
            if origin[axis] < bounds_min[axis] || origin[axis] > bounds_max[axis] {
                return None;
            }
            continue;
        }

        let inv = 1.0 / direction[axis];
        let mut t1 = (bounds_min[axis] - origin[axis]) * inv;
        let mut t2 = (bounds_max[axis] - origin[axis]) * inv;
        let (t_near, t_far, near_on_max) = if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
            (t1, t2, true)
        } else {
            (t1, t2, false)
        };

        if t_near > t_min {
            t_min = t_near;
            enter_axis = axis;
            enter_on_max = near_on_max;
        }
        t_max = t_max.min(t_far);

        if t_max < t_min {
            return None;
        }
    }

    if t_max < 0.0 {
        return None;
    }

    let t_hit = if t_min >= 0.0 { t_min } else { t_max };
    if t_hit < 0.0 {
        return None;
    }

    Some((t_hit, face_index(enter_axis, enter_on_max)))
}

fn face_index(axis: usize, on_max: bool) -> usize {
    match (axis, on_max) {
        (2, true) => 0,  // front (+Z)
        (0, false) => 1, // left (-X)
        (2, false) => 2, // back (-Z)
        (0, true) => 3,  // right (+X)
        (1, true) => 4,  // top (+Y)
        (1, false) => 5, // bottom (-Y)
        _ => 0,
    }
}

pub fn world_ray_to_local(ray: &Ray, object_matrix: &glm::Mat4) -> Option<(glm::Vec3, glm::Vec3)> {
    let inv = glm::inverse(object_matrix);
    let local_origin = (inv * glm::vec4(ray.origin.x, ray.origin.y, ray.origin.z, 1.0)).xyz();
    let local_direction = glm::normalize(
        &(inv * glm::vec4(ray.direction.x, ray.direction.y, ray.direction.z, 0.0)).xyz(),
    );
    Some((local_origin, local_direction))
}

pub fn local_hit_distance_on_ray(
    object_matrix: &glm::Mat4,
    local_origin: glm::Vec3,
    local_direction: glm::Vec3,
    local_t: f32,
    ray: &Ray,
) -> f32 {
    let local_hit = local_origin + local_direction * local_t;
    let world_hit = (object_matrix * glm::vec4(local_hit.x, local_hit.y, local_hit.z, 1.0)).xyz();
    glm::dot(&(world_hit - ray.origin), &ray.direction)
}

pub fn cell_range_for_face(cells_per_side: &[usize; 6], face: usize) -> std::ops::Range<usize> {
    let mut start = 0;
    for (index, &count) in cells_per_side.iter().enumerate() {
        if index == face {
            return start..start + count;
        }
        start += count;
    }
    start..start
}
