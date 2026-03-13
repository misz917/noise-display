use crate::{
    cli::Args, display::Display, extract_frames::extract_frames_with_ffmpeg,
    randomisation_strategy::black_white::BlackWhiteStrategy, screen_buffer::ScreenBuffer,
};
use clap::Parser;
use std::{fs, path::PathBuf, str::FromStr};

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
    let args = Args::parse();

    if let Some(path) = args.path {
        if let Some(extension) = path.extension() {
            let screen_buffer = ScreenBuffer::new(BUFFER_WIDTH, BUFFER_HEIGHT);
            let mut display = Display::new(screen_buffer);
            display.set_noise_strategy(Box::new(BlackWhiteStrategy));

            match extension.to_str().unwrap() {
                "jpg" => {
                    display.run(&path);
                }
                "mp4" => {
                    _ = fs::create_dir(TEMP_FILE_PATH);
                    extract_frames_with_ffmpeg(&path, &PathBuf::from_str(TEMP_FILE_PATH).unwrap())
                        .unwrap();

                    display.run(&PathBuf::from_str(TEMP_FILE_PATH).unwrap());
                    fs::remove_dir_all(TEMP_FILE_PATH).unwrap();
                }
                _ => {}
            }
        }
    }
}
