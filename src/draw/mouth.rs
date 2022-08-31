use image::{RgbImage};

use crate::utils;

pub fn draw(mut img: RgbImage, expression: &str) -> RgbImage {

  let happy_points: [(u32, u32); 6] = [
    (14, 19), (15, 20), (16, 20), (17, 20), (18, 20), (19, 19)
  ];
  let sad_points: [(u32, u32); 6] = [
    (14, 20), (15, 19), (16, 19), (17, 19), (18, 19), (19, 20)
  ];

  if expression == "happy" {
    img = utils::create_pixel::create(img, Box::new(happy_points), [0, 0, 0]);
  }
  if expression == "sad" {
    img = utils::create_pixel::create(img, Box::new(sad_points), [0, 0, 0]);
  }

  return img;
}