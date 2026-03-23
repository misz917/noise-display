use crate::{
    cli::Cli,
    image_source::{ImageSource, single_image_source::SingleImageSource},
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
const BINARIZATION_THRESHOLD: u8 = 127;
const TEMP_FILE_PATH: &str = "./temp/";

fn main() {
    // Cli::run();
    let mut noise_display = NoiseDisplay::new();
    let image_source = SingleImageSource::new(
        &PathBuf::from_str("/home/mil/Code/noise-display/apple.jpg").unwrap(),
    );
    noise_display.display(Box::new(image_source));
}
