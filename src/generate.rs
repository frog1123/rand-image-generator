extern crate image;
extern crate rand;

use std::time::Instant;
use std::path::Path;
use std::fs;
use colored::Colorize;
use image::{ImageBuffer, RgbImage};
use rand::Rng;

use crate::draw;
use crate::utils;

pub struct CustomizationOptions<'a> {
  pub variant: &'a str,
  pub bg_color: [u8; 3],
  pub nose_type: &'a str,
  pub body_color: [u8; 3],
  pub eye_color: [u8; 3],
  pub mouth_accessory: &'a str,
  pub hat_accessory: &'a str,
  pub eye_accessory: &'a str
}

struct VariantInfo<'a> {
  name: &'a str,
  nose_type: &'a str,
  body_colors: Box<[[u8; 3]]>,
  eye_colors: Box<[[u8; 3]]>
}

pub struct Settings {
  pub amount: u32,
  pub starting_value: u32,
  pub logging: bool
}

pub fn generate_images(settings: Settings) -> Result<(), std::io::Error> {
  if !Path::new("./outputs").exists() {
    println!("{} | path: './outputs' doesn't exist, creating path", "✘".red() );
    fs::create_dir_all("./outputs")?;
  }

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
      body_colors: Box::new([[117, 68, 25], [141, 85, 36], [198, 134, 66], [224, 172, 105], [241, 194, 125], [255, 219, 172]]),
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

  let possible_mouth_accessories: [&str; 5] = ["cigarette", "vape", "pipe", "mask", "none"];
  // TODO: add wizard, cowboy, and graduation hat
  let possible_hat_accessories: [&str; 9] = ["tophat", "crown", "beanie", "froghat", "propeller", "cap", "akutsuki_cap", "headband", "none"];
  // TODO: add pirate eye patch
  let possible_eye_accessories: [&str; 4] = ["sunglasses", "monocle", "3d_glasses", "none"];

  // randomize options
  let variant = &variants[rand::thread_rng().gen_range(0..variants.len())];

  let options = CustomizationOptions {
    variant: &variant.name,
    bg_color: [51, 51, 51],
    nose_type: &variant.nose_type,
    body_color: variant.body_colors[rand::thread_rng().gen_range(0..variant.body_colors.len())],
    eye_color: variant.eye_colors[rand::thread_rng().gen_range(0..variant.eye_colors.len())],
    mouth_accessory: &possible_mouth_accessories[rand::thread_rng().gen_range(0..possible_mouth_accessories.len())],
    hat_accessory: &possible_hat_accessories[rand::thread_rng().gen_range(0..possible_hat_accessories.len())],
    eye_accessory: &possible_eye_accessories[rand::thread_rng().gen_range(0..possible_eye_accessories.len())],
  };

  let mut has_hat: bool = true;
  if options.hat_accessory == "headband" { has_hat = false }
  if options.hat_accessory == "none" { has_hat = false }

  // draw image
  let finished_img;
  let img_with_bg = draw::Bg::draw(img, options.bg_color);
  let img_with_body = draw::Body::draw(img_with_bg, &variant.name, options.body_color, has_hat);
  let img_with_mouth = draw::Mouth::draw(img_with_body);
  let img_with_eyes = draw::Eyes::draw(img_with_mouth, &variant.name, options.eye_color, options.body_color);
  let img_with_nose = draw::Nose::draw(img_with_eyes, options.nose_type, options.body_color);
  let img_with_mouth_accessory = draw::Mouth_Accessory::draw(img_with_nose, options.mouth_accessory);
  let img_with_hat_accessory = draw::Hat_Accessory::draw(img_with_mouth_accessory, options.hat_accessory);
  let img_with_eye_accessory = draw::Eye_Accessory::draw(img_with_hat_accessory, options.eye_accessory, options.variant);

  finished_img = img_with_eye_accessory;

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

pub fn generate_specific(options: CustomizationOptions) -> Result<(), std::io::Error> {
  let now = Instant::now();
  let img: RgbImage = ImageBuffer::new(30, 30);

  let mut has_hat: bool = true;
  if options.hat_accessory == "headband" { has_hat = false }
  if options.hat_accessory == "none" { has_hat = false }

  let finished_img;
  let img_with_bg = draw::Bg::draw(img, options.bg_color);
  let img_with_body = draw::Body::draw(img_with_bg, options.variant, options.body_color, has_hat);
  let img_with_mouth = draw::Mouth::draw(img_with_body);
  let img_with_eyes = draw::Eyes::draw(img_with_mouth, options.variant, options.eye_color, options.body_color);
  let img_with_nose = draw::Nose::draw(img_with_eyes, options.nose_type, options.body_color);
  let img_with_mouth_accessory = draw::Mouth_Accessory::draw(img_with_nose, options.mouth_accessory);
  let img_with_hat_accessory = draw::Hat_Accessory::draw(img_with_mouth_accessory, options.hat_accessory);
  let img_with_eye_accessory = draw::Eye_Accessory::draw(img_with_hat_accessory, options.eye_accessory, options.variant);

  finished_img = img_with_eye_accessory;

  // save img
  let path = format!("./outputs/specific.png");
  finished_img.save(Path::new(&path)).unwrap();

  let elapsed = now.elapsed();

  utils::Log_Info::log(&finished_img, &options, elapsed);

  Ok(())
}