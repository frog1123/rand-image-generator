use image::{RgbImage};

pub fn create(mut img: RgbImage, points: Box<[(u32, u32)]>, color: [u8; 3]) -> RgbImage {
  for &point in points.iter() {
    let (x, y) = point;
    *img.get_pixel_mut(x, y) = image::Rgb(color);
  }

  return img;
}