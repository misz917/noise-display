use crate::cli::Cli;

pub mod cli;
pub mod color;
pub mod display;
pub mod extract_frames;
pub mod into_binary;
pub mod randomisation_strategy;
pub mod screen_buffer;

const BUFFER_WIDTH: usize = 160;
const BUFFER_HEIGHT: usize = 90;
const FPS: usize = 20;
const BINARIZATION_THRESHOLD: u8 = 127;
const TEMP_FILE_PATH: &str = "./temp/";

fn main() {
    Cli::run();
}
