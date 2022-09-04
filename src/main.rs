use generate::CustomizationOptions;

use crate::generate::Settings;

mod draw;
mod utils;
mod generate;

fn main() -> Result<(), std::io::Error> {
  // ? use generate_images to generate many or generate_specific to define custom attributes
  // ? generated images will appear in an autogenerated folder, 'outputs'

  generate::generate_images(Settings { amount: 1, starting_value: 1, logging: true }).ok();

  // ? colors are an rgb array of u8
  // ? result be named 'specifc.png'
  let custom_options = CustomizationOptions {
    variant: "alien",                     // possible variants: human, zombie, demon, monkey, alien
    bg_color: [51, 51, 51],               
    nose_type: "alien",                   // possible types: default, monkey, alien
    body_color: [99, 198, 164],           
    eye_color: [255, 255, 255],           
    mouth_accessory: "vape",              // possible types: cigarette, vape, pipe, mask, none
    hat_accessory: "akutsuki_cap",        // possible types: tophat, crown, beanie, froghat, propeller, cap, akutsuki_cap, headband, none
    eye_accessory: "sunglasses"           // possible types: sunglasses, monocle, 3d_glasses, none
  };

  // * make sure to uncomment this
  generate::generate_specific(custom_options).ok();

  Ok(())
}