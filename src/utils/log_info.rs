use std::time::{Duration};
use image::{RgbImage};
use colored::Colorize;
pub use crate::generate::CustomizationOptions;


pub fn log(img: &RgbImage, options: &CustomizationOptions, time: Duration) {
  let (width, height) = img.dimensions();
  println!(
    "{icon} | created img in {time:.3?}: dimensions: {width}×{height}, variant: {variant}, bg_color: {bg_color:?}, nose_type: {nose_type} body_color: {body_color:?}, eye_color: {eye_color:?}, mouth_accessory: {mouth_accessory}, hat_accessory: {hat_accessory}", 
    icon="✔".green(),
    time=time,
    width=width,
    height=height,
    variant=options.variant,
    bg_color=options.bg_color,
    nose_type=options.nose_type,
    body_color=options.body_color,
    eye_color=options.eye_color,
    mouth_accessory=options.mouth_accessory,
    hat_accessory=options.hat_accessory
  );
}