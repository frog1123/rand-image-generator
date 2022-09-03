use crate::generate::Settings;

mod draw;
mod utils;
mod generate;

fn main() -> Result<(), std::io::Error> {
  generate::generate_images(Settings { amount: 1, starting_value: 1, logging: true }).ok();

  Ok(())
}