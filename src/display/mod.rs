use crate::{
    BINARIZATION_THRESHOLD, FPS,
    image_source::{ImageSource, single_image_source::SingleImageSource},
    into_binary::IntoFlatBinary,
    randomisation_strategy::{RandomisationStrategy, black_white::BlackWhiteStrategy},
    screen_buffer::ScreenBuffer,
};
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use std::path::Path;

const SCALE: usize = 20;

const BASE_DIMENSIONS: (usize, usize) = (16, 9);

pub const DEFAULT_WIDTH: usize = BASE_DIMENSIONS.0 * SCALE;
pub const DEFAULT_HEIGHT: usize = BASE_DIMENSIONS.1 * SCALE;

pub struct Display {
    screen_buffer: ScreenBuffer,
    noise_strategy: Box<dyn RandomisationStrategy>,
    mask: Option<Box<[bool]>>,
    window: Window,
    image_source: Option<Box<dyn ImageSource>>,
}

impl Display {
    pub fn new() -> Self {
        let width = DEFAULT_WIDTH;
        let height = DEFAULT_HEIGHT;

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

        Self {
            screen_buffer: ScreenBuffer::new(width, height),
            noise_strategy: Box::new(BlackWhiteStrategy),
            mask: None,
            window,
            image_source: None,
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
        self.noise_strategy.init(&mut self.screen_buffer);
    }

    pub fn run(&mut self, path: &Path) {
        self.image_source = Some(Box::new(SingleImageSource::new(path)));

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

            let image = self.image_source.as_mut().unwrap().next();
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
