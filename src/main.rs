use crate::{
    cli::Args, display::Display, into_binary::IntoFlatBinary,
    randomisation_strategy::black_white::BlackWhiteStrategy, screen_buffer::ScreenBuffer,
};
use clap::Parser;

pub mod cli;
pub mod color;
pub mod display;
pub mod into_binary;
pub mod randomisation_strategy;
pub mod screen_buffer;

const BUFFER_WIDTH: usize = 160;
const BUFFER_HEIGHT: usize = 90;
const FPS: usize = 20;
const BINARIZATION_THRESHOLD: u8 = 127;

fn main() {
    let args = Args::parse();

    let mut mask: Vec<bool> = vec![false; BUFFER_HEIGHT * BUFFER_WIDTH];
    if let Some(path) = args.path {
        if let Some(extension) = path.extension() {
            match extension.to_str().unwrap() {
                "png" => {
                    let mut image = image::open(path).unwrap();
                    mask = image.binarize_and_flatten(BINARIZATION_THRESHOLD);
                }
                "mp4" => {}
                _ => {}
            }
        }
    }

    let screen_buffer = ScreenBuffer::new(BUFFER_WIDTH, BUFFER_HEIGHT);
    let mut display = Display::new(screen_buffer);
    display.set_noise_strategy(Box::new(BlackWhiteStrategy));
    display.set_mask(mask.into());
    display.run();
}
