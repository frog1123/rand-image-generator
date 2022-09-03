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
  variant: &'a str,
  bg_color: [u8; 3],
  nose_type: &'a str,
  body_color: &'a [u8; 3],
  eye_color: &'a [u8; 3],
  mouth_accessory: &'a str,
  hat_accessory: &'a str
}

struct VariantInfo<'a> {
  name: &'a str,
  nose_type: &'a str,
  body_colors: Box<[[u8; 3]]>,
  eye_colors: Box<[[u8; 3]]>
}

struct Settings {
  amount: u32,
  starting_value: u32,
  logging: bool
}

fn main() -> Result<(), std::io::Error> {
  if !Path::new("./outputs").exists() {
    println!("{} | path: './outputs' doesn't exist, creating path", "✘".red() );
    fs::create_dir_all("./outputs")?;
  }

  generate_images(
    Settings { 
      amount: 1, 
      starting_value: 1,
      logging: true 
    }).ok();

  Ok(())
}

fn generate_images(settings: Settings) -> Result<(), std::io::Error> {
  for i in settings.starting_value..(settings.amount + settings.starting_value) {
    generate(i, settings.logging, 0).map_err(|err| println!("{:?}", err)).ok();
  }

  Ok(())
}

fn generate(num: u32, logging: bool, starting_num: u32) -> Result<(), std::io::Error> {
  let now = Instant::now();

  // construct a new RGB ImageBuffer with the specified width and height
  let img: RgbImage = ImageBuffer::new(30, 30);

  let variants: [VariantInfo; 5] = [
    VariantInfo {
      name: "human",
      nose_type: "default",
      body_colors: Box::new([[232, 190, 172], [255, 219, 172], [198, 134, 66]]),
      eye_colors: Box::new([[12, 160, 148], [101, 160, 12], [122, 95, 191]])
    },
    VariantInfo {
      name: "zombie",
      nose_type: "default",
      body_colors: Box::new([[69, 117, 68], [52, 96, 51]]),
      eye_colors: Box::new([[122, 23, 44], [170, 76, 97]])
    },
    VariantInfo {
      name: "demon",
      nose_type: "default",
      body_colors: Box::new([[221, 82, 64], [198, 67, 49]]),
      eye_colors: Box::new([[234, 234, 51], [234, 127, 51]])
    },
    VariantInfo {
      name: "monkey",
      nose_type: "monkey",
      body_colors: Box::new([[66, 51, 33]]),
      eye_colors: Box::new([[232, 232, 232]])
    },
    VariantInfo {
      name: "alien",
      nose_type: "alien",
      body_colors: Box::new([[99, 198, 164], [63, 168, 131]]),
      eye_colors: Box::new([[255, 255, 255]]) // doesnt do anything
    }
  ];

  let possible_mouth_accessories: [&str; 3] = ["cigarette", "pipe", "none"];
  let possible_hat_accessories: [&str; 6] = ["tophat", "crown", "beanie", "froghat", "headband", "none"];

  // randomize options
  let variant = &variants[rand::thread_rng().gen_range(0..variants.len())];

  let options = CustomizationOptions {
    variant: &variant.name,
    bg_color: [51, 51, 51],
    nose_type: &variant.nose_type,
    body_color: &variant.body_colors[rand::thread_rng().gen_range(0..variant.body_colors.len())],
    eye_color: &variant.eye_colors[rand::thread_rng().gen_range(0..variant.eye_colors.len())],
    mouth_accessory: &possible_mouth_accessories[rand::thread_rng().gen_range(0..possible_mouth_accessories.len())],
    hat_accessory: &possible_hat_accessories[rand::thread_rng().gen_range(0..possible_hat_accessories.len())],
  };

  let mut has_hat: bool = true;
  if options.hat_accessory == "headband" { has_hat = false }
  if options.hat_accessory == "none" { has_hat = false }

  // draw image
  let finished_img;
  let img_with_bg = draw::Bg::draw(img, options.bg_color);
  let img_with_body = draw::Body::draw(img_with_bg, &variant.name, *options.body_color, has_hat);
  let img_with_mouth = draw::Mouth::draw(img_with_body);
  let img_with_eyes = draw::Eyes::draw(img_with_mouth, &variant.name, *options.eye_color, *options.body_color);
  let img_with_nose = draw::Nose::draw(img_with_eyes, options.nose_type, *options.body_color);
  let img_with_mouth_accessory = draw::Mouth_Accessory::draw(img_with_nose, options.mouth_accessory);
  let img_with_hat_accessory = draw::Hat_Accesory::draw(img_with_mouth_accessory, options.hat_accessory);

  finished_img = img_with_hat_accessory;

  // save img
  let path = format!("./outputs/{}.png", (num + starting_num).to_string());
  finished_img.save(Path::new(&path)).unwrap();

  // log info
  let elapsed = now.elapsed();
  if logging {
    utils::Log_Info::log(&finished_img, &options, elapsed);
  }

  Ok(())
}