use crate::{
    cli::Args,
    color::{BLACK, WHITE},
};
use clap::Parser;
use image::DynamicImage;
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use rayon::prelude::*;
use std::{path::PathBuf, str::FromStr};

pub mod cli;
pub mod color;

const BUFFER_WIDTH: usize = 160;
const BUFFER_HEIGHT: usize = 90;
const FPS: usize = 20;
const BINARIZATION_THRESHOLD: u8 = 127;

pub trait IntoBinary {
    fn binarize(&mut self, threshold: u8) -> Result<(), ()>;
}

impl IntoBinary for DynamicImage {
    fn binarize(&mut self, threshold: u8) -> Result<(), ()> {
        let mut gray = self.to_luma8();
        let _ = gray.iter_mut().par_bridge().for_each(|pixel| {
            if *pixel > threshold {
                *pixel = WHITE as u8;
            } else {
                *pixel = BLACK as u8;
            }
        });

        *self = DynamicImage::ImageLuma8(gray);
        Ok(())
    }
}

fn main() {
    // let args = Args::parse();
    let file_path = PathBuf::from_str("hello.png").unwrap();
    let mut image = image::open(file_path).unwrap();
    println!("{:?}", image.binarize(BINARIZATION_THRESHOLD));

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

        screen_buffer.par_iter_mut().for_each(|pixel| {
            if rand::random_bool(0.5) {
                *pixel = WHITE;
            } else {
                *pixel = 0;
            }
        });

        window
            .update_with_buffer(&screen_buffer, BUFFER_WIDTH, BUFFER_HEIGHT)
            .unwrap();
    }
}
