use crate::{
    BINARIZATION_THRESHOLD, FPS,
    into_binary::IntoFlatBinary,
    randomisation_strategy::{RandomisationStrategy, black_white::BlackWhiteStrategy},
    screen_buffer::ScreenBuffer,
};
use image::{DynamicImage, imageops::FilterType};
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use std::{collections::LinkedList, fs, path::Path};

pub struct Display {
    screen_buffer: ScreenBuffer,
    noise_strategy: Box<dyn RandomisationStrategy>,
    mask: Option<Box<[bool]>>,
    window: Window,
    memory: LinkedList<DynamicImage>,
}

impl Display {
    pub fn new(screen_buffer: ScreenBuffer) -> Self {
        let width = screen_buffer.width();
        let height = screen_buffer.height();

        let mut window = Window::new(
            "ESC to exit; E to pause; R to resume",
            width,
            height,
            WindowOptions {
                scale: Scale::X8,
                ..WindowOptions::default()
            },
        )
        .unwrap();
        window.set_target_fps(FPS);

        Self {
            screen_buffer,
            noise_strategy: Box::new(BlackWhiteStrategy),
            mask: None,
            window,
            memory: LinkedList::new(),
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
        self.noise_strategy.randomise(&mut self.screen_buffer, None);
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

    fn scale_images_in_memory(&mut self) {
        let width = self.screen_buffer.width() as u32;
        let height = self.screen_buffer.height() as u32;

        let mut list: LinkedList<DynamicImage> = LinkedList::new();

        for (i, image) in self.memory.iter().enumerate() {
            let scaled = image.resize_exact(width, height, FilterType::Nearest);
            list.push_back(scaled);
            print!("\r{}", i);
        }

        self.memory = list;
    }

    pub fn run(&mut self, path: &Path) {
        self.load_images_into_memory(path);
        self.scale_images_in_memory();

        self.init();

        let mut paused = false;
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            if self.window.is_key_pressed(Key::E, KeyRepeat::No) {
                paused = true;
            } else if self.window.is_key_pressed(Key::R, KeyRepeat::No) {
                paused = false;
            }
            if paused {
                self.window.update();
                continue;
            }

            let image = self.memory.pop_front();
            if let Some(image) = image {
                let mask = image.binarize_and_flatten(BINARIZATION_THRESHOLD);
                self.set_mask(mask.into());
            }

            self.noise_strategy
                .randomise(&mut self.screen_buffer, self.mask.as_deref());

            self.window
                .update_with_buffer(
                    self.screen_buffer.get_buffer(),
                    self.screen_buffer.width(),
                    self.screen_buffer.height(),
                )
                .unwrap();
        }
    }
}
