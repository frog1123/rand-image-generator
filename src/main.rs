extern crate image;
extern crate rand;

use std::path::Path;
use std::fs;
use colored::Colorize;
use image::{ImageBuffer, RgbImage};
use rand::Rng;

mod draw;
mod utils;

pub struct CustomizationOptions {
  mouth: String
}

fn main() -> Result<(), std::io::Error> {
  // construct a new RGB ImageBuffer with the specified width and height
  let img: RgbImage = ImageBuffer::new(30, 30);

  // create output of directory if it doesnt exist
  if !Path::new("./outputs").exists() {
    println!("{} | path: './outputs' doesn't exist, creating path", "✘".red() );
    fs::create_dir_all("./outputs")?;
  }

  // randomize options
  let options = CustomizationOptions {
    mouth: if rand::thread_rng().gen_range(0..2) == 1 { "sad".to_string() } else { "happy".to_string() }
  };

  // draw image
  let finished_img;

  let img_with_body = draw::Body::draw(img);
  let img_with_mouth = draw::Mouth::draw(img_with_body, options.mouth.as_str());

  finished_img = img_with_mouth;

  // save img
  finished_img.save(Path::new("./outputs/1.png")).unwrap();

  // log info
  utils::Log_Info::log(&finished_img, &options);

  Ok(())
}