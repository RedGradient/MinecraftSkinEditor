use std::collections::{BTreeMap, HashMap};

use image::{DynamicImage, EncodableLayout, GenericImage, GenericImageView, ImageBuffer, ImageError, Rgba};

use crate::glium_area::body_part::BodyPart;
use crate::glium_area::cube_side::CubeSide;
use crate::glium_area::vertex::Vertex;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}
impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Dimensions {
    width: u32,
    height: u32,
}
impl Dimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Dimensions { width, height }
    }
}

#[derive(Debug)]
struct SideMeta {
    position: Point,
    dimensions: Dimensions,
}
impl SideMeta {
    pub fn new(position: Point, dimensions: Dimensions) -> Self {
        SideMeta { position, dimensions }
    }
}

type HelperMap = HashMap<BodyPart, BTreeMap<CubeSide, SideMeta>>;

pub struct SkinParser {
    helper_map: HelperMap,
}

const VEC_IN_CELL: usize = 4;

pub type CubeSideColors = BTreeMap<CubeSide, Vec<Rgba<u8>>>;
pub type ColorMap = HashMap<BodyPart, CubeSideColors>;

#[derive(Copy, Clone, PartialEq)]
pub enum ModelType {
    Classic,
    Slim
}

pub enum TextureLoadError {
    Image(ImageError),
    ImageDimensionError(String)
}

impl SkinParser {
    pub fn new(model_type: &ModelType) -> Self {
        SkinParser { helper_map: SkinParser::generate_helper_map(model_type) }
    }

    pub fn load_from_path(&self, path: &str) -> Result<ColorMap, TextureLoadError> {
        let img = image::open(path).map_err(|err| TextureLoadError::Image(err))?;
        self.load_image(img)
    }

    pub fn load_from_bytes(&self, bytes: &bytes::Bytes) -> Result<ColorMap, TextureLoadError> {
        let img = image::load_from_memory(bytes.as_bytes()).map_err(|err| TextureLoadError::Image(err))?;
        self.load_image(img)
    }

    fn load_image(&self, img: image::DynamicImage) -> Result<ColorMap, TextureLoadError> {
        if img.dimensions() != (64, 64) {
            let message = format!("Image has wrong dimensions: ({}, {})", img.dimensions().0, img.dimensions().1);
            return Err(TextureLoadError::ImageDimensionError(message));
        }

        let mut color_map: ColorMap = HashMap::new();

        for (body_part, helper) in &self.helper_map {
            let mut body_part_color_map: BTreeMap<CubeSide, Vec<Rgba<u8>>> = BTreeMap::new();
            for (side, meta) in helper {
                let colors = self.image_slice(&img, meta.position.x, meta.position.y, meta.dimensions.width, meta.dimensions.height);
                body_part_color_map.insert(side.clone(), colors);
            }
            color_map.insert(body_part.clone(), body_part_color_map);
        }

        Ok(color_map)
    }
    
    pub fn export_as(
        &self,
        body_part: &BodyPart,
        imgbuf: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        vertexes: &Vec<Vertex>
    )
    {
        let body_part_helper = self.helper_map.get(body_part).unwrap();
        let mut vec_passed = 0;
        for (index, (_, meta)) in body_part_helper.iter().enumerate() {
            let vec_count = meta.dimensions.width as usize * meta.dimensions.height as usize * VEC_IN_CELL;
            let range = vec_passed..vec_passed + vec_count;
            let pixels = vertexes[range]
                .chunks(VEC_IN_CELL)
                .map(|chunk| {
                    let pixel = chunk.first().unwrap().color;
                    SkinParser::from_f32_to_u8_pixel(pixel)
                })
                .collect();
            self.export_pixels(imgbuf, &pixels, meta.position.x, meta.position.y, meta.dimensions.width, meta.dimensions.height);
            vec_passed += vec_count;
        }
    }

    fn image_slice(&self, img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> Vec<Rgba<u8>> {
        let mut slice = vec![];
        for i in y..y + height {
            for j in x..x + width {
                let pixel = img.get_pixel(j, i);
                slice.push(pixel);
            }
        }
        slice
    }

    fn export_pixels(
        &self,
        imgbuf: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        pixels: &Vec<Rgba<u8>>,
        x: u32,
        y: u32,
        width: u32,
        height: u32
    )
    {
        for i in 0..height {
            for j in 0..width {
                let index = (i * width + j) as usize;
                let pixel = pixels[index];
                imgbuf.put_pixel(x + j, y + i, pixel);
            }
        }
    }

    fn from_f32_to_u8_pixel(pixel: [f32; 4]) -> Rgba<u8> {
        Rgba([
            (pixel[0] * 255.0).round() as u8,
            (pixel[1] * 255.0).round() as u8,
            (pixel[2] * 255.0).round() as u8,
            (pixel[3] * 255.0).round() as u8,
        ])
    }

    fn generate_helper_map(model_type: &ModelType) -> HelperMap {
        let mut helper_map: HelperMap = HashMap::new();

        let mut head_helper: BTreeMap<CubeSide, SideMeta> = BTreeMap::new();
        {
            head_helper.insert(CubeSide::Front, SideMeta::new(Point::new(8, 8), Dimensions::new(8, 8)));
            head_helper.insert(CubeSide::Left, SideMeta::new(Point::new(16, 8), Dimensions::new(8, 8)));
            head_helper.insert(CubeSide::Back, SideMeta::new(Point::new(24, 8), Dimensions::new(8, 8)));
            head_helper.insert(CubeSide::Right, SideMeta::new(Point::new(0, 8), Dimensions::new(8, 8)));
            head_helper.insert(CubeSide::Top, SideMeta::new(Point::new(8, 0), Dimensions::new(8, 8)));
            head_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(16, 0), Dimensions::new(8, 8)));
        }

        let mut torso_helper: BTreeMap<CubeSide, SideMeta> = BTreeMap::new();
        {
            torso_helper.insert(CubeSide::Front, SideMeta::new(Point::new(20, 20), Dimensions::new(8, 12)));
            torso_helper.insert(CubeSide::Left, SideMeta::new(Point::new(28, 20), Dimensions::new(4, 12)));
            torso_helper.insert(CubeSide::Back, SideMeta::new(Point::new(32, 20), Dimensions::new(8, 12)));
            torso_helper.insert(CubeSide::Right, SideMeta::new(Point::new(16, 20), Dimensions::new(4, 12)));
            torso_helper.insert(CubeSide::Top, SideMeta::new(Point::new(20, 16), Dimensions::new(8, 4)));
            torso_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(28, 16), Dimensions::new(8, 4)));
        }

        let mut right_arm_helper: BTreeMap<CubeSide, SideMeta> = BTreeMap::new();
        match model_type {
            ModelType::Classic => {
                right_arm_helper.insert(CubeSide::Front, SideMeta::new(Point::new(44, 20), Dimensions::new(4, 12)));
                right_arm_helper.insert(CubeSide::Left, SideMeta::new(Point::new(48, 20), Dimensions::new(4, 12)));
                right_arm_helper.insert(CubeSide::Back, SideMeta::new(Point::new(52, 20), Dimensions::new(4, 12)));
                right_arm_helper.insert(CubeSide::Right, SideMeta::new(Point::new(40, 20), Dimensions::new(4, 12)));
                right_arm_helper.insert(CubeSide::Top, SideMeta::new(Point::new(44, 16), Dimensions::new(4, 4)));
                right_arm_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(48, 16), Dimensions::new(4, 4)));
            },
            ModelType::Slim => {
                right_arm_helper.insert(CubeSide::Front, SideMeta::new(Point::new(44, 20), Dimensions::new(3, 12)));
                right_arm_helper.insert(CubeSide::Left, SideMeta::new(Point::new(47, 20), Dimensions::new(4, 12)));
                right_arm_helper.insert(CubeSide::Back, SideMeta::new(Point::new(51, 20), Dimensions::new(3, 12)));
                right_arm_helper.insert(CubeSide::Right, SideMeta::new(Point::new(40, 20), Dimensions::new(4, 12)));
                right_arm_helper.insert(CubeSide::Top, SideMeta::new(Point::new(44, 16), Dimensions::new(3, 4)));
                right_arm_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(47, 16), Dimensions::new(3, 4)));
            }
        }

        let mut left_arm_helper: BTreeMap<CubeSide, SideMeta> = BTreeMap::new();
        match model_type {
            ModelType::Classic => {
                left_arm_helper.insert(CubeSide::Front, SideMeta::new(Point::new(36, 52), Dimensions::new(4, 12)));
                left_arm_helper.insert(CubeSide::Left, SideMeta::new(Point::new(40, 52), Dimensions::new(4, 12)));
                left_arm_helper.insert(CubeSide::Back, SideMeta::new(Point::new(44, 52), Dimensions::new(4, 12)));
                left_arm_helper.insert(CubeSide::Right, SideMeta::new(Point::new(32, 52), Dimensions::new(4, 12)));
                left_arm_helper.insert(CubeSide::Top, SideMeta::new(Point::new(36, 48), Dimensions::new(4, 4)));
                left_arm_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(40, 48), Dimensions::new(4, 4)));
            },
            ModelType::Slim => {
                left_arm_helper.insert(CubeSide::Front, SideMeta::new(Point::new(36, 52), Dimensions::new(3, 12)));
                left_arm_helper.insert(CubeSide::Left, SideMeta::new(Point::new(39, 52), Dimensions::new(4, 12)));
                left_arm_helper.insert(CubeSide::Back, SideMeta::new(Point::new(43, 52), Dimensions::new(3, 12)));
                left_arm_helper.insert(CubeSide::Right, SideMeta::new(Point::new(32, 52), Dimensions::new(4, 12)));
                left_arm_helper.insert(CubeSide::Top, SideMeta::new(Point::new(36, 48), Dimensions::new(3, 4)));
                left_arm_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(39, 48), Dimensions::new(3, 4)));
            }
        }

        let mut right_leg_helper: BTreeMap<CubeSide, SideMeta> = BTreeMap::new();
        {
            right_leg_helper.insert(CubeSide::Front, SideMeta::new(Point::new(4, 20), Dimensions::new(4, 12)));
            right_leg_helper.insert(CubeSide::Left, SideMeta::new(Point::new(8, 20), Dimensions::new(4, 12)));
            right_leg_helper.insert(CubeSide::Back, SideMeta::new(Point::new(12, 20), Dimensions::new(4, 12)));
            right_leg_helper.insert(CubeSide::Right, SideMeta::new(Point::new(0, 20), Dimensions::new(4, 12)));
            right_leg_helper.insert(CubeSide::Top, SideMeta::new(Point::new(4, 16), Dimensions::new(4, 4)));
            right_leg_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(8, 16), Dimensions::new(4, 4)));
        }

        let mut left_leg_helper: BTreeMap<CubeSide, SideMeta> = BTreeMap::new();
        {
            left_leg_helper.insert(CubeSide::Front, SideMeta::new(Point::new(20, 52), Dimensions::new(4, 12)));
            left_leg_helper.insert(CubeSide::Left, SideMeta::new(Point::new(24, 52), Dimensions::new(4, 12)));
            left_leg_helper.insert(CubeSide::Back, SideMeta::new(Point::new(28, 52), Dimensions::new(4, 12)));
            left_leg_helper.insert(CubeSide::Right, SideMeta::new(Point::new(16, 52), Dimensions::new(4, 12)));
            left_leg_helper.insert(CubeSide::Top, SideMeta::new(Point::new(20, 48), Dimensions::new(4, 4)));
            left_leg_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(24, 48), Dimensions::new(4, 4)));
        }

        let mut head_outer_helper: BTreeMap<CubeSide, SideMeta> = BTreeMap::new();
        {
            head_outer_helper.insert(CubeSide::Front, SideMeta::new(Point::new(40, 8), Dimensions::new(8, 8)));
            head_outer_helper.insert(CubeSide::Left, SideMeta::new(Point::new(48, 8), Dimensions::new(8, 8)));
            head_outer_helper.insert(CubeSide::Back, SideMeta::new(Point::new(56, 8), Dimensions::new(8, 8)));
            head_outer_helper.insert(CubeSide::Right, SideMeta::new(Point::new(32, 8), Dimensions::new(8, 8)));
            head_outer_helper.insert(CubeSide::Top, SideMeta::new(Point::new(40, 0), Dimensions::new(8, 8)));
            head_outer_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(48, 0), Dimensions::new(8, 8)));
        }

        let mut torso_outer_helper: BTreeMap<CubeSide, SideMeta> = BTreeMap::new();
        {
            torso_outer_helper.insert(CubeSide::Front, SideMeta::new(Point::new(20, 36), Dimensions::new(8, 12)));
            torso_outer_helper.insert(CubeSide::Left, SideMeta::new(Point::new(28, 36), Dimensions::new(4, 12)));
            torso_outer_helper.insert(CubeSide::Back, SideMeta::new(Point::new(32, 36), Dimensions::new(8, 12)));
            torso_outer_helper.insert(CubeSide::Right, SideMeta::new(Point::new(16, 36), Dimensions::new(4, 12)));
            torso_outer_helper.insert(CubeSide::Top, SideMeta::new(Point::new(20, 32), Dimensions::new(8, 4)));
            torso_outer_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(28, 32), Dimensions::new(8, 4)));
        }

        let mut right_arm_outer_helper: BTreeMap<CubeSide, SideMeta> = BTreeMap::new();
        match model_type {
            ModelType::Classic => {
                right_arm_outer_helper.insert(CubeSide::Front, SideMeta::new(Point::new(44, 36), Dimensions::new(4, 12)));
                right_arm_outer_helper.insert(CubeSide::Left, SideMeta::new(Point::new(48, 36), Dimensions::new(4, 12)));
                right_arm_outer_helper.insert(CubeSide::Back, SideMeta::new(Point::new(52, 36), Dimensions::new(4, 12)));
                right_arm_outer_helper.insert(CubeSide::Right, SideMeta::new(Point::new(40, 36), Dimensions::new(4, 12)));
                right_arm_outer_helper.insert(CubeSide::Top, SideMeta::new(Point::new(44, 32), Dimensions::new(4, 4)));
                right_arm_outer_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(48, 32), Dimensions::new(4, 4)));
            },
            ModelType::Slim => {
                right_arm_outer_helper.insert(CubeSide::Front, SideMeta::new(Point::new(44, 36), Dimensions::new(3, 12)));
                right_arm_outer_helper.insert(CubeSide::Left, SideMeta::new(Point::new(47, 36), Dimensions::new(4, 12)));
                right_arm_outer_helper.insert(CubeSide::Back, SideMeta::new(Point::new(51, 36), Dimensions::new(3, 12)));
                right_arm_outer_helper.insert(CubeSide::Right, SideMeta::new(Point::new(40, 36), Dimensions::new(4, 12)));
                right_arm_outer_helper.insert(CubeSide::Top, SideMeta::new(Point::new(44, 32), Dimensions::new(3, 4)));
                right_arm_outer_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(47, 32), Dimensions::new(3, 4)));
            }
        }
        let mut left_arm_outer_helper: BTreeMap<CubeSide, SideMeta> = BTreeMap::new();
        match model_type {
            ModelType::Classic => {
                left_arm_outer_helper.insert(CubeSide::Front, SideMeta::new(Point::new(52, 52), Dimensions::new(4, 12)));
                left_arm_outer_helper.insert(CubeSide::Left, SideMeta::new(Point::new(56, 52), Dimensions::new(4, 12)));
                left_arm_outer_helper.insert(CubeSide::Back, SideMeta::new(Point::new(60, 52), Dimensions::new(4, 12)));
                left_arm_outer_helper.insert(CubeSide::Right, SideMeta::new(Point::new(48, 52), Dimensions::new(4, 12)));
                left_arm_outer_helper.insert(CubeSide::Top, SideMeta::new(Point::new(52, 48), Dimensions::new(4, 4)));
                left_arm_outer_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(56, 48), Dimensions::new(4, 4)));
            },
            ModelType::Slim => {
                left_arm_outer_helper.insert(CubeSide::Front, SideMeta::new(Point::new(52, 52), Dimensions::new(3, 12)));
                left_arm_outer_helper.insert(CubeSide::Left, SideMeta::new(Point::new(55, 52), Dimensions::new(4, 12)));
                left_arm_outer_helper.insert(CubeSide::Back, SideMeta::new(Point::new(59, 52), Dimensions::new(3, 12)));
                left_arm_outer_helper.insert(CubeSide::Right, SideMeta::new(Point::new(48, 52), Dimensions::new(4, 12)));
                left_arm_outer_helper.insert(CubeSide::Top, SideMeta::new(Point::new(52, 48), Dimensions::new(3, 4)));
                left_arm_outer_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(55, 48), Dimensions::new(3, 4)));
            }
        }

        let mut right_leg_outer_helper: BTreeMap<CubeSide, SideMeta> = BTreeMap::new();
        {
            right_leg_outer_helper.insert(CubeSide::Front, SideMeta::new(Point::new(4, 36), Dimensions::new(4, 12)));
            right_leg_outer_helper.insert(CubeSide::Left, SideMeta::new(Point::new(8, 36), Dimensions::new(4, 12)));
            right_leg_outer_helper.insert(CubeSide::Back, SideMeta::new(Point::new(12, 36), Dimensions::new(4, 12)));
            right_leg_outer_helper.insert(CubeSide::Right, SideMeta::new(Point::new(0, 36), Dimensions::new(4, 12)));
            right_leg_outer_helper.insert(CubeSide::Top, SideMeta::new(Point::new(4, 32), Dimensions::new(4, 4)));
            right_leg_outer_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(8, 32), Dimensions::new(4, 4)));
        }

        let mut left_leg_outer_helper: BTreeMap<CubeSide, SideMeta> = BTreeMap::new();
        {
            left_leg_outer_helper.insert(CubeSide::Front, SideMeta::new(Point::new(4, 52), Dimensions::new(4, 12)));
            left_leg_outer_helper.insert(CubeSide::Left, SideMeta::new(Point::new(8, 52), Dimensions::new(4, 12)));
            left_leg_outer_helper.insert(CubeSide::Back, SideMeta::new(Point::new(12, 52), Dimensions::new(4, 12)));
            left_leg_outer_helper.insert(CubeSide::Right, SideMeta::new(Point::new(0, 52), Dimensions::new(4, 12)));
            left_leg_outer_helper.insert(CubeSide::Top, SideMeta::new(Point::new(4, 48), Dimensions::new(4, 4)));
            left_leg_outer_helper.insert(CubeSide::Bottom, SideMeta::new(Point::new(8, 48), Dimensions::new(4, 4)));
        }

        helper_map.insert(BodyPart::Head, head_helper);
        helper_map.insert(BodyPart::Torso, torso_helper);
        helper_map.insert(BodyPart::RightArm, right_arm_helper);
        helper_map.insert(BodyPart::LeftArm, left_arm_helper);
        helper_map.insert(BodyPart::RightLeg, right_leg_helper);
        helper_map.insert(BodyPart::LeftLeg, left_leg_helper);
        helper_map.insert(BodyPart::HeadOuter, head_outer_helper);
        helper_map.insert(BodyPart::TorsoOuter, torso_outer_helper);
        helper_map.insert(BodyPart::RightArmOuter, right_arm_outer_helper);
        helper_map.insert(BodyPart::LeftArmOuter, left_arm_outer_helper);
        helper_map.insert(BodyPart::RightLegOuter, right_leg_outer_helper);
        helper_map.insert(BodyPart::LeftLegOuter, left_leg_outer_helper);

        helper_map
    }
}
