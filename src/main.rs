use crate::{
    cli::Args,
    color::{BLACK, WHITE},
    into_binary::IntoFlatBinary,
    screen_buffer::ScreenBuffer,
};
use clap::Parser;
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use rayon::prelude::*;

pub mod cli;
pub mod color;
pub mod into_binary;
pub mod randomisation_strategy;
pub mod screen_buffer;

const BUFFER_WIDTH: usize = 160;
const BUFFER_HEIGHT: usize = 90;
const FPS: usize = 20;
const BINARIZATION_THRESHOLD: u8 = 127;

fn main() {
    let args = Args::parse();

    let mut flat_binary: Vec<bool> = vec![false; BUFFER_HEIGHT * BUFFER_WIDTH];
    if let Some(path) = args.path {
        if let Some(extension) = path.extension() {
            match extension.to_str().unwrap() {
                "png" => {
                    let mut image = image::open(path).unwrap();
                    flat_binary = image.binarize_and_flatten(BINARIZATION_THRESHOLD);
                }
                "mp4" => {}
                "txt" => {}
                _ => {}
            }
        }
    }

    let mut window = Window::new(
        "ESC to exit; E to pause; R to resume",
        BUFFER_WIDTH,
        BUFFER_HEIGHT,
        WindowOptions {
            scale: Scale::X8,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    window.set_target_fps(FPS);

    let mut screen_buffer = [0u32; BUFFER_WIDTH * BUFFER_HEIGHT];
    screen_buffer.par_iter_mut().for_each(|pixel| {
        *pixel = rand::random_range(0..=0xFFFFFF);
        // if rand::random_bool(0.5) {
        //     *pixel = WHITE;c
        // } else {
        //     *pixel = BLACK;
        // }
    });

    let mut paused = false;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::E, KeyRepeat::No) {
            paused = true;
        } else if window.is_key_pressed(Key::R, KeyRepeat::No) {
            paused = false;
        }
        if paused {
            window.update();
            continue;
        }

        screen_buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pixel)| {
                if !(args.negative ^ flat_binary[i]) {
                    *pixel = rand::random_range(0..=0xFFFFFF);
                    // if rand::random_bool(0.5) {
                    //     *pixel = WHITE;
                    // } else {
                    //     *pixel = BLACK;
                    // }
                }
            });

        window
            .update_with_buffer(&screen_buffer, BUFFER_WIDTH, BUFFER_HEIGHT)
            .unwrap();
    }
}
