use image::{RgbImage};
use colored::Colorize;
pub use crate::CustomizationOptions;


pub fn log(img: &RgbImage, options: &CustomizationOptions) {
  let (width, height) = img.dimensions();
  println!(
    "{icon} | created img: width: {width}, height: {height}, bg_color: {bg_color:?}, body_color: {body_color:?}, mouth_type: {mouth_type}, eye_color: {eye_color:?}", 
    icon="✔".green(),
    width=width,
    height=height,
    bg_color=options.bg_color,
    body_color=options.body_color,
    mouth_type=options.mouth,
    eye_color=options.eye_color
  );
}
