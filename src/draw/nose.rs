use image::{RgbImage};

use crate::utils;

pub fn draw(mut img: RgbImage, nose: &String, color: [u8; 3]) -> RgbImage {
  let default_points: [(u32, u32); 1] = [(16, 18)];
  let monkey_points: [(u32, u32); 2] = [(15, 18), (17, 18)];
  let alien_points: [(u32, u32); 3] = [(16, 17), (16, 18), (16, 19)];

  let darkened_color = utils::Darken_Color::darken(color, 20);

  if nose.as_str() == "monkey" {
    img = utils::create_pixel::create(img, Box::new(monkey_points), [0, 0, 0]);
  } else {
    if nose == "alien" {
      img = utils::create_pixel::create(img, Box::new(alien_points), darkened_color);
    }
    else {
      img = utils::create_pixel::create(img, Box::new(default_points), darkened_color);
    }
  }

  return img;
}