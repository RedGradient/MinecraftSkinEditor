use gtk::gdk::RGBA;
use rand::Rng;

use crate::glium_area::body_part::BodyPart;
use crate::glium_area::body_part::BodyPart::*;
use crate::glium_area::body_part::BodyPart::RightArmOuter;
use crate::glium_area::model_object::ModelObject;
use crate::glium_area::skin_parser::ModelType;

pub fn rgba_to_f32(rgba: RGBA) -> [f32; 4] {
    [rgba.red(), rgba.green(), rgba.blue(), rgba.alpha()]
}
pub fn f32_to_rgba(color: [f32; 4]) -> RGBA {
    RGBA::new(color[0], color[1], color[2], color[3])
}

pub fn guess_model_type(bytes: &[u8]) -> Result<ModelType, ()> {
    Ok(ModelType::Classic)
}

pub fn random_brightness(color: [f32; 4]) -> [f32; 4] {
    let mut rng = rand::thread_rng();
    
    let mut hsv = rgba_to_hsv(color);
    
    let v = hsv[2];
    let adjustments = if v > 0.8 {
        get_adjustments(-0.10, 1.0 - v, 0.025)
    } else {
        get_adjustments(0.0, 0.20, 0.025)
    };
    
    let idx = rng.gen_range(0..adjustments.len());
    let new_v = adjustments[idx] + hsv[2];

    hsv[2] = new_v;
    hsv_to_rgba(hsv)
}

fn get_adjustments(start: f32, stop: f32, step: f32) -> Vec<f32> {
    let mut adjustments = vec![];
    let mut current = start;
    while current < stop {
        adjustments.push(current);
        current += step;
    }
    
    adjustments
}

fn rgba_to_hsv(rgba: [f32; 4]) -> [f32; 4] {
    let (r, g, b, a) = (rgba[0], rgba[1], rgba[2], rgba[3]);

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let saturation = if max == 0.0 {
        0.0
    } else {
        delta / max
    };

    let value = max;

    [hue / 360.0, saturation, value, a]
}

fn hsv_to_rgba(hsv: [f32; 4]) -> [f32; 4] {
    let (h, s, v, a) = (hsv[0] * 360.0, hsv[1], hsv[2], hsv[3]);

    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    [r + m, g + m, b + m, a]
}

pub fn classic_to_slim_arm(classic_arm: &ModelObject, slim_arm: &mut ModelObject, arm_side: BodyPart) {
    let colors = classic_arm.get_pixels();
    let (front, right, back, left, top, bottom) = match arm_side {
        LeftArm | LeftArmOuter => ((0, 2), (48, 51), (97, 99), (144, 147), (192, 194), (208, 210)),
        RightArm | RightArmOuter => ((1, 3), (48, 51), (96, 98), (144, 147), (193, 195), (209, 211)),
        _ => panic!("Wrong arm type passed to the function")
    };

    move_classic_to_slim_by_side(slim_arm, &colors, front.0, front.1, 12, 0);     // FRONT
    move_classic_to_slim_by_side(slim_arm, &colors, right.0, right.1, 12, 36);    // RIGHT
    move_classic_to_slim_by_side(slim_arm, &colors, back.0, back.1, 12, 84);      // BACK
    move_classic_to_slim_by_side(slim_arm, &colors, left.0, left.1, 12, 120);     // LEFT
    move_classic_to_slim_by_side(slim_arm, &colors, top.0, top.1, 4, 168);        // TOP
    move_classic_to_slim_by_side(slim_arm, &colors, bottom.0, bottom.1, 4, 180);  // BOTTOM
}

fn move_classic_to_slim_by_side(
    target_arm: &mut ModelObject,
    colors: &Vec<[f32; 4]>,
    start: usize,
    stop: usize,
    side_height: usize,
    mut cell_index: usize,
) {
    let indexes: Vec<(usize, usize)> = (0..side_height).map(|n| (start + n * 4, stop + n * 4)).collect();
    let colors: Vec<[f32; 4]> = indexes.iter()
        .flat_map(|(start, stop)| colors[*start..=*stop].to_vec())
        .collect();
    for color in colors {
        target_arm.paint(cell_index, color);
        cell_index += 1;
    }
}

pub fn slim_to_classic_arm(slim_arm: &ModelObject, classic_arm: &mut ModelObject, arm_side: BodyPart) {
    let colors = slim_arm.get_pixels();

    let (front, right, back, left, top, bottom) = match arm_side {
        LeftArm | LeftArmOuter => ((0, 2), (48, 51), (97, 99), (144, 147), (192, 194), (208, 210)),
        RightArm | RightArmOuter => ((1, 3), (48, 51), (96, 98), (144, 147), (193, 195), (209, 211)),
        _ => panic!("Wrong arm type passed to the function")
    };

    move_slim_to_classic_by_side(classic_arm, &colors, front.0, front.1, 12, 0);     // FRONT
    move_slim_to_classic_by_side(classic_arm, &colors, right.0, right.1, 12, 36);    // RIGHT
    move_slim_to_classic_by_side(classic_arm, &colors, back.0, back.1, 12, 84);      // BACK
    move_slim_to_classic_by_side(classic_arm, &colors, left.0, left.1, 12, 120);     // LEFT
    move_slim_to_classic_by_side(classic_arm, &colors, top.0, top.1, 4, 168);        // TOP
    move_slim_to_classic_by_side(classic_arm, &colors, bottom.0, bottom.1, 4, 180);  // BOTTOM
}

fn move_slim_to_classic_by_side(
    target_arm: &mut ModelObject,
    colors: &Vec<[f32; 4]>,
    start: usize,
    stop: usize,
    side_height: usize,
    mut color_index: usize
) {
    let cells: Vec<usize> = (0..side_height).flat_map(|n| (start + n * 4)..=(stop + n * 4)).collect();
    for cell in cells {
        target_arm.paint(cell, colors[color_index]);
        color_index += 1;
    }
}

pub fn generate_random_filename() -> String {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = std::iter::repeat(())
        .map(|()| rng.sample(rand::distributions::Alphanumeric))
        .take(10)
        .collect();

    let mut filename = String::from_utf8(random_bytes).expect("Error creating random filename");
    filename.push_str(".png");
    filename
}