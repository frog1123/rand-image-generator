use image::{RgbImage};

use crate::utils;

pub fn draw(mut img: RgbImage, color: [u8; 3]) -> RgbImage {

  let eyes_points: [(u32, u32); 2] = [
    (14, 15), (18, 15)
  ];
  let eyes_colored_points: [(u32, u32); 2] = [
    (15, 15), (19, 15)
  ];

  img = utils::create_pixel::create(img, Box::new(eyes_points), [0, 0, 0]);
  img = utils::create_pixel::create(img, Box::new(eyes_colored_points), color);

  return img;
}