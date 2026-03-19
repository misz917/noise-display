use crate::{
    BINARIZATION_THRESHOLD, FPS,
    into_binary::IntoFlatBinary,
    randomisation_strategy::{RandomisationStrategy, black_white::BlackWhiteStrategy},
    screen_buffer::{self, ScreenBuffer},
};
use image::{DynamicImage, imageops::FilterType};
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use std::{collections::LinkedList, fs, path::Path};

const SCALE: usize = 20;

const BASE_DIMENSIONS: (usize, usize) = (16, 9);

const DEFAULT_WIDTH: usize = BASE_DIMENSIONS.0 * SCALE;
const DEFAULT_HEIGHT: usize = BASE_DIMENSIONS.1 * SCALE;

pub struct Display {
    screen_buffer: Option<ScreenBuffer>,
    noise_strategy: Box<dyn RandomisationStrategy>,
    mask: Option<Box<[bool]>>,
    window: Option<Window>,
    memory: LinkedList<DynamicImage>,
}

impl Display {
    pub fn new() -> Self {
        Self {
            noise_strategy: Box::new(BlackWhiteStrategy),
            mask: None,
            memory: LinkedList::new(),
            screen_buffer: None,
            window: None,
        }
    }

    #[inline]
    pub fn set_noise_strategy(&mut self, noise_strategy: Box<dyn RandomisationStrategy>) {
        self.noise_strategy = noise_strategy;
    }

    #[inline]
    pub fn set_mask(&mut self, mask: Box<[bool]>) {
        self.mask = Some(mask);
    }

    fn init(&mut self) {
        self.noise_strategy
            .init(self.screen_buffer.as_mut().unwrap());
    }

    fn load_images_into_memory(&mut self, path: &Path) {
        if path.is_dir() {
            let paths = fs::read_dir(path).unwrap();
            for (i, file_name) in paths.map(|f| f.unwrap().file_name()).enumerate() {
                let image = image::open(path.join(file_name)).unwrap();
                self.memory.push_back(image);
                print!("\r{}", i);
            }
        } else {
            let image = image::open(path).unwrap();
            self.memory.push_back(image);
        }
    }

    fn initialize_screen_buffer(&mut self) -> bool {
        if let Some(front) = self.memory.front() {
            let width = front.width();
            let height = front.height();
            let buffer = ScreenBuffer::new(width as usize, height as usize);
            self.screen_buffer = Some(buffer);
            return true;
        }
        return false;
    }

    fn initialize_window(&mut self) {
        if let Some(screen_buffer) = &self.screen_buffer {
            let width = screen_buffer.width();
            let height = screen_buffer.height();

            let mut window = Window::new(
                "ESC to exit; E to pause; R to resume",
                width,
                height,
                WindowOptions {
                    scale: Scale::X4,
                    ..WindowOptions::default()
                },
            )
            .unwrap();
            window.set_target_fps(FPS);
        }
    }

    pub fn run(&mut self, path: &Path) {
        self.load_images_into_memory(path);

        self.init();

        if let Some(window) = self.window.as_mut() {
            let mut paused = false;
            // if let Some(mut screen_buffer) = self.screen_buffer.as_mut() {
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

                let image = self.memory.pop_front();
                if let Some(image) = image {
                    let mask = image.binarize_and_flatten(BINARIZATION_THRESHOLD);
                    self.set_mask(mask.into());
                }

                self.noise_strategy
                    .randomise(&mut screen_buffer, self.mask.as_deref());

                window
                    .update_with_buffer(
                        self.screen_buffer.as_mut().unwrap().get_buffer(),
                        self.screen_buffer.as_mut().unwrap().width(),
                        self.screen_buffer.as_mut().unwrap().height(),
                    )
                    .unwrap();
            }
            // }
        }
    }
}
