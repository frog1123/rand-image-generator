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

pub struct CustomizationOptions {
  bg_color: [u8; 3],
  body_color: [u8; 3],
  mouth: String,
  eye_color: [u8; 3]
}

fn main() -> Result<(), std::io::Error> {
  if !Path::new("./outputs").exists() {
    println!("{} | path: './outputs' doesn't exist, creating path", "✘".red() );
    fs::create_dir_all("./outputs")?;
  }

  for i in 1..6 {
    generate(i, true).map_err(|err| println!("{:?}", err)).ok();
  }

  Ok(())
}

fn generate(num: i32, logging: bool) -> Result<(), std::io::Error> {
  let now = Instant::now();

  // construct a new RGB ImageBuffer with the specified width and height
  let img: RgbImage = ImageBuffer::new(30, 30);

  // create output of directory if it doesnt exist

  let body_colors: [[u8; 3]; 6] = [
    [232, 190, 172], [255, 219, 172], [198, 134, 66], [89, 46, 24], [30, 204, 76], [78, 216, 207]
  ];

  let eye_colors: [[u8; 3]; 4] = [
    [12, 160, 148], [101, 160, 12], [122, 95, 191], [68, 22, 22]
  ];

  let bg_colors: [[u8; 3]; 4] = [
    [255, 133, 91], [39, 234, 133], [183, 129, 209], [83, 93, 132]
  ];

  // randomize options
  let options = CustomizationOptions {
    bg_color: bg_colors[rand::thread_rng().gen_range(0..4)],
    body_color: body_colors[rand::thread_rng().gen_range(0..6)],
    mouth: if rand::thread_rng().gen_range(0..2) == 1 { "sad".to_string() } else { "happy".to_string() },
    eye_color: eye_colors[rand::thread_rng().gen_range(0..4)]
  };

  // draw image
  let finished_img;

  let img_with_bg = draw::Bg::draw(img, options.bg_color);
  let img_with_body = draw::Body::draw(img_with_bg, options.body_color);
  let img_with_mouth = draw::Mouth::draw(img_with_body, options.mouth.as_str());
  let img_with_eyes = draw::Eyes::draw(img_with_mouth, options.eye_color);

  finished_img = img_with_eyes;

  
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