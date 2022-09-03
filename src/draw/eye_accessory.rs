use image::{RgbImage};

use crate::utils;

pub fn draw(mut img: RgbImage, acc_type: &str, variant: &str) -> RgbImage {
  if acc_type == "none" { return img }

  let mut sh = 14;
  let mut mh = 14;
  let mut g3h = 14;
  if variant == "alien" || variant == "monkey" { sh = 13; mh = 13; g3h = 13 }

  let sunglasses_points: [(u32, u32); 25] = [
    (12, sh), (13, sh), (14, sh), (15, sh), (16, sh), (17, sh), (18, sh), (19, sh), (20, sh), (21, sh), (22, sh), (23, sh), (24, sh),
    (13, sh+1), (14, sh+1), (15, sh+1), (18, sh+1), (19, sh+1), (20, sh+1),
    (13, sh+2), (14, sh+2), (15, sh+2), (18, sh+2), (19, sh+2), (20, sh+2)
  ];
  let monocle_points_1: [(u32, u32); 12] = [(18, mh), (19, mh), (17, mh+1), (20, mh+1), (17, mh+2), (20, mh+2), (18, mh+3), (19, mh+3), (20, mh+3), (20, mh+4), (20, mh+5), (20, mh+6)];
  let monocle_points_2: [(u32, u32); 4] = [(18, mh+1), (19, mh+1), (18, mh+2), (19, mh+2)];
  let glasses_3d_points_1: [(u32, u32); 25] = [
    (12, g3h), (13, g3h), (14, g3h), (15, g3h), (16, g3h), (17, g3h), (18, g3h), (19, g3h), (20, g3h), (21, g3h), (22, g3h), (23, g3h), (24, g3h),
    (12, g3h+1), (15, g3h+1), (17, g3h+1), (20, g3h+1),
    (12, g3h+2), (13, g3h+2), (14, g3h+2), (15, g3h+2), (17, g3h+2), (18, g3h+2), (19, g3h+2), (20, g3h+2)
  ];

  if acc_type == "sunglasses" {
    img = utils::create_pixel::create(img, Box::new(sunglasses_points), [0, 0, 0]);
  }
  if acc_type == "monocle" {
    img = utils::create_pixel::create(img, Box::new(monocle_points_1), [219, 185, 32]);
    img = utils::create_pixel::create(img, Box::new(monocle_points_2), [32, 203, 219]);
  }
  if acc_type == "3d_glasses" {
    img = utils::create_pixel::create(img, Box::new(glasses_3d_points_1), [222, 222, 222]);
    *img.get_pixel_mut(13, g3h+1) = image::Rgb([237, 66, 66]);
    *img.get_pixel_mut(14, g3h+1) = image::Rgb([237, 66, 66]);
    *img.get_pixel_mut(18, g3h+1) = image::Rgb([66, 197, 237]);
    *img.get_pixel_mut(19, g3h+1) = image::Rgb([66, 197, 237]);
  }

  return img
}