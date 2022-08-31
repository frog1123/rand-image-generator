extern crate image;
extern crate rand;

use std::time::Instant;
use std::path::Path;
use std::fs;
use colored::Colorize;
use image::{ImageBuffer, RgbImage};
use rand::Rng;

mod draw;
mod utils;

pub struct CustomizationOptions<'a> {
  variant: &'a String,
  bg_color: [u8; 3],
  nose_type: &'a String,
  body_color: &'a [u8; 3],
  eye_color: &'a [u8; 3]
}

pub struct VariantInfo {
  name: String,
  nose_type: String,
  body_colors: Box<[[u8; 3]]>,
  eye_colors: Box<[[u8; 3]]>
}

fn main() -> Result<(), std::io::Error> {
  if !Path::new("./outputs").exists() {
    println!("{} | path: './outputs' doesn't exist, creating path", "✘".red() );
    fs::create_dir_all("./outputs")?;
  }

  for i in 1..2 {
    generate(i, true).map_err(|err| println!("{:?}", err)).ok();
  }

  Ok(())
}

fn generate(num: i32, logging: bool) -> Result<(), std::io::Error> {
  let now = Instant::now();

  // construct a new RGB ImageBuffer with the specified width and height
  let img: RgbImage = ImageBuffer::new(30, 30);

  let variants: [VariantInfo; 4] = [
    VariantInfo {
      name: "human".to_string(),
      nose_type: "default".to_string(),
      body_colors: Box::new([[232, 190, 172], [255, 219, 172], [198, 134, 66]]),
      eye_colors: Box::new([[12, 160, 148], [101, 160, 12], [122, 95, 191]])
    },
    VariantInfo {
      name: "zombie".to_string(),
      nose_type: "default".to_string(),
      body_colors: Box::new([[69, 117, 68], [52, 96, 51]]),
      eye_colors: Box::new([[122, 23, 44], [170, 76, 97]])
    },
    VariantInfo {
      name: "monkey".to_string(),
      nose_type: "monkey".to_string(),
      body_colors: Box::new([[66, 51, 33]]),
      eye_colors: Box::new([[232, 232, 232]])
    },
    VariantInfo {
      name: "alien".to_string(),
      nose_type: "alien".to_string(),
      body_colors: Box::new([[99, 198, 164], [63, 168, 131]]),
      eye_colors: Box::new([[255, 255, 255]]) // doesnt do anything
    }
  ];

  // randomize options
  let variant = &variants[rand::thread_rng().gen_range(0..variants.len())];

  let options = CustomizationOptions {
    variant: &variant.name,
    bg_color: [51, 51, 51],
    nose_type: &variant.nose_type,
    body_color: &variant.body_colors[rand::thread_rng().gen_range(0..variant.body_colors.len())],
    eye_color: &variant.eye_colors[rand::thread_rng().gen_range(0..variant.eye_colors.len())]
  };

  // draw image
  let finished_img;
  let img_with_bg = draw::Bg::draw(img, options.bg_color);
  let img_with_body = draw::Body::draw(img_with_bg, &variant.name, *options.body_color);
  let img_with_mouth = draw::Mouth::draw(img_with_body);
  let img_with_eyes = draw::Eyes::draw(img_with_mouth, &variant.name, *options.eye_color, *options.body_color);
  let img_with_nose = draw::Nose::draw(img_with_eyes, options.nose_type, *options.body_color);
  finished_img = img_with_nose;

  // save img
  let path = format!("./outputs/{}.png", num.to_string());
  finished_img.save(Path::new(&path)).unwrap();

  // log info
  let elapsed = now.elapsed();
  if logging {
    utils::Log_Info::log(&finished_img, &options, elapsed);
  }

  Ok(())
}