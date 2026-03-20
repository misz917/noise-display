use crate::cli::Cli;

pub mod cli;
pub mod color;
pub mod display;
pub mod extract_frames;
pub mod image_source;
pub mod into_binary;
pub mod randomisation_strategy;
pub mod screen_buffer;

const FPS: usize = 30;
const BINARIZATION_THRESHOLD: u8 = 127;
const TEMP_FILE_PATH: &str = "./temp/";

fn main() {
    Cli::run();
}
