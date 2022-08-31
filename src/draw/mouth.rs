use image::{RgbImage};

use crate::utils;

pub fn draw(mut img: RgbImage) -> RgbImage {

  let mouth_points: [(u32, u32); 5] = [
    (14, 21), (15, 21), (16, 21), (17, 21), (18, 21)
  ];

  img = utils::create_pixel::create(img, Box::new(mouth_points), [0, 0, 0]);

  return img;
}