use generate::CustomizationOptions;

use crate::generate::Settings;

mod draw;
mod utils;
mod generate;

fn main() -> Result<(), std::io::Error> {
  // ? use generate_images to generate many or generate_specific to define custom attributes

  generate::generate_images(Settings { amount: 1, starting_value: 1, logging: true }).ok();

  // ? colors are an rgb array of u8
  // ? result be called 'specifc.png'
  let custom_options = CustomizationOptions {
    variant: "alien",                     // possible variants: human, zombie, demon, monkey, alien
    bg_color: [100, 100, 100],            //
    nose_type: "alien",                   // possible types: default, monkey, alient
    body_color: [99, 198, 164],           //
    eye_color: [255, 255, 255],           //
    mouth_accessory: "pipe",              // possible types: cigarette, vape, pipe, none
    hat_accessory: "cap",                 // possible types: tophat, crown, beanie, froghat, propeller, cap, headband, none
    eye_accessory: "sunglasses"           // possible types: sunglasses, monocle, 3d_glasses, none
  };

  // * make sure to uncomment this
  // generate::generate_specific(custom_options).ok();

  Ok(())
}