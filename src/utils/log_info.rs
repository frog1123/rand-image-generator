use image::{RgbImage};
use colored::Colorize;
pub use crate::CustomizationOptions;


pub fn log(img: &RgbImage, options: &CustomizationOptions) {
  let (width, height) = img.dimensions();
  println!(
    "{icon} | created img: width: {width}, height: {height}, mouth_type: {mouth_type}", 
    icon="✔".green(),
    width=width,
    height=height,
    mouth_type=options.mouth
  );
}
