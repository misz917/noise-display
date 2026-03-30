use crate::{
    cli::Cli,
    image_source::image_source_factory::ImageSourceFactory,
    noise_display::{NoiseDisplay, interface::NoiseDisplayInterface},
};
use std::{path::PathBuf, str::FromStr};

pub mod cli;
pub mod color;
pub mod display;
pub mod extract_frames;
pub mod image_source;
pub mod into_binary;
pub mod randomisation_strategy;
pub mod screen_buffer;

pub mod noise_display;

const FPS: usize = 30;
const DEFAULT_TARGET_FPS: usize = 30;
const BINARIZATION_THRESHOLD: u8 = 127;
const TEMP_FILE_PATH: &str = "./temp/";

fn main() {
    // Cli::run();
    let mut noise_display = NoiseDisplay::default();
    let path = PathBuf::from_str("/home/mil/Code/noise-display/bad_apple.mp4").unwrap();
    let image_source = ImageSourceFactory::new_image_source(&path);
    noise_display.display(image_source);
}
