use image::{RgbImage};

use crate::utils;

pub fn draw(mut img: RgbImage, variant: &str, color: [u8; 3], body_color: [u8; 3]) -> RgbImage {
  let mut height: u32 = 16;
  if variant == "monkey" || variant == "alien" { height = 15 }

  let eyes_points: [(u32, u32); 2] = [(14, height), (19, height)];
  let eyes_colored_points: [(u32, u32); 2] = [(13, height), (18, height)];

  let alien_points_1: [(u32, u32); 4] = [(13, height), (18, height), (14, height - 1), (19, height - 1)];
  let alien_points_2: [(u32, u32); 4] = [(14, height), (19, height), (13, height - 1), (18, height - 1)];

  if variant == "alien" {
    img = utils::create_pixel::create(img, Box::new(alien_points_1), [0, 0, 0]);
    img = utils::create_pixel::create(img, Box::new(alien_points_2), utils::Darken_Color::darken(body_color, 20));
  } else {
    img = utils::create_pixel::create(img, Box::new(eyes_points), [0, 0, 0]);
    img = utils::create_pixel::create(img, Box::new(eyes_colored_points), color);
  }

  return img;
}