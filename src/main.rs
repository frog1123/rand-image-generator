extern crate image;

use std::path::Path;
use std::fs;
use colored::Colorize;
use image::{ImageBuffer, RgbImage};

mod draw;

fn main() -> Result<(), std::io::Error> {
  // construct a new RGB ImageBuffer with the specified width and height
  let img: RgbImage = ImageBuffer::new(50, 50);

  // create output of directory if it doesnt exist
  if !Path::new("./outputs").exists() {
    println!("{} path: './outputs' doesn't exist, creating path", "✘".red() );
    fs::create_dir_all("./outputs")?;
  }

  // draw image
  let finished_img = draw::Hi::draw(img);

  // save img
  finished_img.save(Path::new("./outputs/1.png")).unwrap();


  let (width, height) = finished_img.dimensions();
  println!("{} created img | width: {}, height: {}", "✔".green(), width, height);

  Ok(())
}