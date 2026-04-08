use crate::cli::Cli;

pub mod cli;
pub mod color;
pub mod extract_frames;
pub mod image_source;
pub mod into_binary;
pub mod noise_strategy;
pub mod screen_buffer;

pub mod noise_display;

const SCALE: usize = 20;

const BASE_DIMENSIONS: (usize, usize) = (16, 9);

pub const DEFAULT_WIDTH: usize = BASE_DIMENSIONS.0 * SCALE;
pub const DEFAULT_HEIGHT: usize = BASE_DIMENSIONS.1 * SCALE;

const DEFAULT_TARGET_FPS: usize = 30;
const BINARIZATION_THRESHOLD: u8 = 127;

fn main() {
    Cli::new_run().unwrap();
}
