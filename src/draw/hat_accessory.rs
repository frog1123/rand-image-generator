use image::{RgbImage};

use crate::utils;

pub fn draw(mut img: RgbImage, acc_type: &str) -> RgbImage {
  if acc_type == "none" { return img }

  let tophat_points_1: [(u32, u32); 141] = [
    (11, 1), (12, 1), (13, 1), (14, 1), (15, 1), (16, 1), (17, 1), (18, 1), (19, 1), (20, 1), (21, 1), (22, 1), (23, 1), (24, 1), (25, 1),
    (11, 2), (12, 2), (13, 2), (14, 2), (15, 2), (16, 2), (17, 2), (18, 2), (19, 2), (20, 2), (21, 2), (22, 2), (23, 2), (24, 2), (25, 2),
    (11, 3), (12, 3), (13, 3), (14, 3), (15, 3), (16, 3), (17, 3), (18, 3), (19, 3), (20, 3), (21, 3), (22, 3), (23, 3), (24, 3), (25, 3),
    (11, 4), (12, 4), (13, 4), (14, 4), (15, 4), (16, 4), (17, 4), (18, 4), (19, 4), (20, 4), (21, 4), (22, 4), (23, 4), (24, 4), (25, 4),
    (11, 5), (12, 5), (13, 5), (14, 5), (15, 5), (16, 5), (17, 5), (18, 5), (19, 5), (20, 5), (21, 5), (22, 5), (23, 5), (24, 5), (25, 5),
    (11, 6), (12, 6), (13, 6), (14, 6), (15, 6), (16, 6), (17, 6), (18, 6), (19, 6), (20, 6), (21, 6), (22, 6), (23, 6), (24, 6), (25, 6),
    (11, 7), (12, 7), (13, 7), (14, 7), (15, 7), (16, 7), (17, 7), (18, 7), (19, 7), (20, 7), (21, 7), (22, 7), (23, 7), (24, 7), (25, 7),
    (11, 8), (12, 8), (13, 8), (14, 8), (15, 8), (16, 8), (17, 8), (18, 8), (19, 8), (20, 8), (21, 8), (22, 8), (23, 8), (24, 8), (25, 8),
    (11, 9), (25, 9),
    (9, 10), (10, 10), (11, 10), (12, 10), (13, 10), (14, 10), (15, 10), (16, 10), (17, 10), (18, 10), (19, 10), (20, 10), (21, 10), (22, 10), (23, 10), (24, 10), (25, 10), (26, 10), (27, 10)
  ];
  let tophat_points_2: [(u32, u32); 13] = [
   (12, 9), (13, 9), (14, 9), (15, 9), (16, 9), (17, 9), (18, 9), (19, 9), (20, 9), (21, 9), (22, 9), (23, 9), (24, 9)
  ];
  let crown_points: [(u32, u32); 25] = [
    (14, 4), (18, 4), (22, 4),
    (14, 5), (15, 5), (17, 5), (19, 5), (21, 5), (22, 5),
    (14, 6), (16, 6), (17, 6), (18, 6), (19, 6), (20, 6), (22, 6),
    (14, 7), (15, 7), (16, 7), (17, 7), (18, 7), (19, 7), (20, 7), (21, 7), (22, 7)
  ];
  let beanie_points_1: [(u32, u32); 21] = [
    (10, 9), (10, 8), (10, 7), (11, 6), (12, 5), 
    (13, 4), (14, 3), (15, 3), (16, 3), (17, 3), (18, 3), (19, 3), (20, 3), (21, 3), (22, 3),
    (23, 4), (24, 5), (25, 6), (26, 7), (26, 8), (26, 9)
  ];
  let beanie_points_2: [(u32, u32); 55] = [
    (14, 4), (15, 4), (16, 4), (17, 4), (18, 4), (19, 4), (20, 4), (21, 4), (22, 4),
    (13, 5), (14, 5), (15, 5), (16, 5), (17, 5), (18, 5), (19, 5), (20, 5), (21, 5), (22, 5), (23, 5),
    (12, 6), (13, 6), (14, 6), (15, 6), (16, 6), (17, 6), (18, 6), (19, 6), (20, 6), (21, 6), (22, 6), (23, 6), (24, 6),
    (11, 7), (12, 7), (13, 7), (14, 7), (15, 7), (16, 7), (17, 7), (18, 7), (19, 7), (20, 7), (21, 7), (22, 7), (23, 7), (24, 7), (25, 7),
    (12, 9), (14, 9), (16, 9), (18, 9), (20, 9), (22, 9), (24, 9)
  ];
  let beanie_points_3: [(u32, u32); 23] = [
    (11, 8), (12, 8), (13, 8), (14, 8), (15, 8), (16, 8), (17, 8), (18, 8), (19, 8), (20, 8), (21, 8), (22, 8), (23, 8), (24, 8), (25, 8),
    (11, 9), (13, 9), (15, 9), (17, 9), (19, 9), (21, 9), (23, 9), (25, 9)
  ];
  let froghat_points_1: [(u32, u32); 132] = [
    (12, 4), (12, 4), (13, 4), (14, 4), (15, 4), (16, 4), (17, 4), (18, 4), (19, 4), (20, 4), (21, 4), (22, 4), (23, 4), (24, 4),
    (11, 5), (12, 5), (12, 5), (13, 5), (14, 5), (15, 5), (16, 5), (17, 5), (18, 5), (19, 5), (20, 5), (21, 5), (22, 5), (23, 5), (24, 5), (25, 5),
    (11, 6), (12, 6), (12, 6), (13, 6), (14, 6), (15, 6), (16, 6), (17, 6), (18, 6), (19, 6), (20, 6), (21, 6), (22, 6), (23, 6), (24, 6), (25, 6),
    (11, 7), (12, 7), (12, 7), (13, 7), (14, 7), (15, 7), (16, 7), (17, 7), (18, 7), (19, 7), (20, 7), (21, 7), (22, 7), (23, 7), (24, 7), (25, 7),
    (11, 8), (12, 8), (12, 8), (13, 8), (14, 8), (15, 8), (16, 8), (17, 8), (18, 8), (19, 8), (20, 8), (21, 8), (22, 8), (23, 8), (24, 8), (25, 8),
    (11, 9), (12, 9), (12, 9), (13, 9), (14, 9), (15, 9), (16, 9), (17, 9), (18, 9), (19, 9), (20, 9), (21, 9), (22, 9), (23, 9), (24, 9), (25, 9),
    (10, 10), (11, 10), (12, 10), (12, 10), (13, 10), (14, 10), (15, 10), (16, 10), (17, 10), (18, 10), (19, 10), (20, 10), (21, 10), (22, 10), (23, 10), (24, 10), (25, 10), (26, 10),
    (9, 11), (10, 11), (11, 11), (12, 11), (12, 11), (13, 11), (14, 11), (15, 11), (16, 11), (17, 11), (18, 11), (19, 11), (20, 11), (21, 11), (22, 11), (23, 11), (24, 11), (25, 11), (26, 11), (27, 11)
  ];
  let froghat_points_2: [(u32, u32); 8] = [
    (14, 3), (15, 3), (21, 3), (22, 3),
    (14, 4), (15, 4), (21, 4), (22, 4)
  ];
  let froghat_points_3: [(u32, u32); 7] = [
    (15, 4), (21, 4), (16, 6), (17, 7), (18, 7), (19, 7), (20, 6)
  ];
  let headband_points_1: [(u32, u32); 13] = [(12, 11), (13, 11), (14, 11), (15, 11), (16, 11), (17, 11), (18, 11), (19, 11), (20, 11), (21, 11), (22, 11), (23, 11), (24, 11)];
  let headband_points_2: [(u32, u32); 13] = [(12, 12), (13, 12), (14, 12), (15, 12), (16, 12), (17, 12), (18, 12), (19, 12), (20, 12), (21, 12), (22, 12), (23, 12), (24, 12)];

  if acc_type == "tophat" {
    img = utils::create_pixel::create(img, Box::new(tophat_points_1), [0, 0, 0]);
    img = utils::create_pixel::create(img, Box::new(tophat_points_2), [255, 255, 255]);
  }
  if acc_type == "crown" {
    img = utils::create_pixel::create(img, Box::new(crown_points), [237, 220, 42]);
    *img.get_pixel_mut(15, 6) = image::Rgb([42, 220, 237]);
    *img.get_pixel_mut(21, 6) = image::Rgb([42, 220, 237]);
    *img.get_pixel_mut(18, 5) = image::Rgb([239, 59, 43]);
  }
  if acc_type == "beanie" {
    img = utils::create_pixel::create(img, Box::new(beanie_points_1), [0, 0, 0]);
    img = utils::create_pixel::create(img, Box::new(beanie_points_2), [186, 87, 29]);
    img = utils::create_pixel::create(img, Box::new(beanie_points_3), [145, 69, 26]);
  }
  if acc_type == "froghat" {
    img = utils::create_pixel::create(img, Box::new(froghat_points_1), [96, 216, 56]);
    img = utils::create_pixel::create(img, Box::new(froghat_points_2), [222, 222, 222]);
    img = utils::create_pixel::create(img, Box::new(froghat_points_3), [0, 0, 0]);
  }
  if acc_type == "headband" {
    img = utils::create_pixel::create(img, Box::new(headband_points_1), [222, 222, 222]);
    img = utils::create_pixel::create(img, Box::new(headband_points_2), [49, 181, 183]);
  }

  return img;
}