use clap::Parser;
use minifb::{Key, KeyRepeat, Window, WindowOptions};

use crate::cli::Args;

pub mod cli;

const BUFFER_WIDTH: usize = 160;
const BUFFER_HEIGHT: usize = 90;

const SCREEN_WIDTH: usize = BUFFER_WIDTH * 3;
const SCREEN_HEIGHT: usize = BUFFER_HEIGHT * 3;

fn main() {
    // let args = Args::parse();

    let mut window = Window::new(
        "ESC to exit; SPACE to pause",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap();

    window.set_target_fps(60);

    let mut small_buffer = [0u32; BUFFER_WIDTH * BUFFER_HEIGHT];
    let mut screen_buffer = [0u32; SCREEN_WIDTH * SCREEN_HEIGHT];

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

        for pixel in screen_buffer.iter_mut() {
            if rand::random_bool(0.5) {
                let red = 255 << 16;
                let green = 255 << 8;
                let blue = 255;
                let color = red | green | blue;
                *pixel = color;
            } else {
                *pixel = 0;
            }
        }

        window
            .update_with_buffer(&screen_buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}
