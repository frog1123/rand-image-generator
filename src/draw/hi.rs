use image::{RgbImage};

pub fn draw(mut img: RgbImage) -> RgbImage {
  *img.get_pixel_mut(5, 5) = image::Rgb([255, 255, 255]);
  *img.get_pixel_mut(5, 6) = image::Rgb([255, 255, 255]);
  *img.get_pixel_mut(5, 7) = image::Rgb([255, 255, 255]);
  *img.get_pixel_mut(5, 8) = image::Rgb([255, 255, 255]);
  *img.get_pixel_mut(5, 9) = image::Rgb([255, 255, 255]);

  *img.get_pixel_mut(6, 7) = image::Rgb([255, 255, 255]);

  *img.get_pixel_mut(7, 5) = image::Rgb([255, 255, 255]);
  *img.get_pixel_mut(7, 6) = image::Rgb([255, 255, 255]);
  *img.get_pixel_mut(7, 7) = image::Rgb([255, 255, 255]);
  *img.get_pixel_mut(7, 8) = image::Rgb([255, 255, 255]);
  *img.get_pixel_mut(7, 9) = image::Rgb([255, 255, 255]);

  *img.get_pixel_mut(9, 5) = image::Rgb([255, 255, 255]);
  *img.get_pixel_mut(9, 6) = image::Rgb([255, 255, 255]);
  *img.get_pixel_mut(9, 7) = image::Rgb([255, 255, 255]);
  *img.get_pixel_mut(9, 8) = image::Rgb([255, 255, 255]);
  *img.get_pixel_mut(9, 9) = image::Rgb([255, 255, 255]);

  return img;
}