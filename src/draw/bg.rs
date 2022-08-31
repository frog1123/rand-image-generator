use image::{RgbImage, Rgb};

pub fn draw(mut img: RgbImage, color: [u8; 3]) -> RgbImage {

  for x in 0..30 {
    for y in 0..30 {
        img.put_pixel(x, y, Rgb(color));
    }
  }

  return img;
}