use image::{RgbImage};

use crate::utils;

pub fn draw(mut img: RgbImage, acc_type: &str) -> RgbImage {

  if acc_type == "none" { return img }

  let cigarette_points_1: [(u32, u32); 5] = [(13, 21), (12, 21), (11, 21), (10, 21), (9, 21)];
  let cigarette_points_2: [(u32, u32); 4] = [(8, 19), (8, 18), (8, 17), (8, 16)];

  let pipe_points_1: [(u32, u32); 23] = [
    (13, 22), (12, 23), (11, 24), (10, 24), (9, 24), (9, 23), (9, 22), (8, 22), (7, 22), (6, 22), (5, 22),
    (5, 23), (5, 24), (6, 25), (7, 26), (8, 26), (9, 26), (10, 26), (11, 26), (12, 25), (13, 24), (14, 23), (15, 22)
  ];
  let pipe_points_2: [(u32, u32); 14] = [
    (14, 22), (13, 23), (12, 24), (11, 25), (10, 25), (9, 25), (8, 25), (7, 25), (8, 24), 
    (7, 24), (6, 24), (8, 23), (7, 23), (6, 23)
  ];
  let pipe_points_3: [(u32, u32); 9] = [
    (7, 20), (7, 18), (6, 16), (7, 16), (8, 16), (6, 15), (7, 15), (8, 15), (7, 14)
  ];
  let mask_points_1: [(u32, u32); 33] = [
    (13, 18), (14, 18), (15, 18), (16, 18), (17, 18), (18, 18), (19, 18), (19, 19), (19, 20), (19, 21), (19, 22), (18, 23), (17, 23), (16, 23), (15, 23), (14, 23), (13, 22), (13, 21), (13, 20), (13, 19), (13, 18),
    (12, 17), (20, 17), (21, 17), (22, 17), (23, 16), (24, 16),
    (12, 21), (20, 21), (21, 21), (22, 21), (23, 21), (24, 21)
  ];
  let mask_points_2: [(u32, u32); 10] = [(14, 19), (15, 19), (16, 19), (17, 19), (18, 19), (14, 21), (15, 21), (16, 21), (17, 21), (18, 21)];
  let mask_points_3: [(u32, u32); 10] = [(14, 20), (15, 20), (16, 20), (17, 20), (18, 20), (14, 22), (15, 22), (16, 22), (17, 22), (18, 22)];

  if acc_type == "cigarette" {
    img = utils::create_pixel::create(img, Box::new(cigarette_points_1), [181, 181, 181]);
    img = utils::create_pixel::create(img, Box::new(cigarette_points_2), [150, 150, 150]);
    *img.get_pixel_mut(8, 21) = image::Rgb([226, 131, 63]);
  }
  if acc_type == "vape" {
    img = utils::create_pixel::create(img, Box::new(cigarette_points_1), [25, 25, 25]);
    img = utils::create_pixel::create(img, Box::new(cigarette_points_2), [150, 150, 150]);
    *img.get_pixel_mut(8, 21) = image::Rgb([12, 183, 175]);
  }
  if acc_type == "pipe" {
    img = utils::create_pixel::create(img, Box::new(pipe_points_1), [0, 0, 0]);
    img = utils::create_pixel::create(img, Box::new(pipe_points_2), [84, 47, 21]);
    img = utils::create_pixel::create(img, Box::new(pipe_points_3), [150, 150, 150]);
  }
  if acc_type == "mask" {
    img = utils::create_pixel::create(img, Box::new(mask_points_1), [225, 225, 225]);
    img = utils::create_pixel::create(img, Box::new(mask_points_2), [128, 206, 203]);
    img = utils::create_pixel::create(img, Box::new(mask_points_3), [120, 186, 183]);
  }

  return img;
}