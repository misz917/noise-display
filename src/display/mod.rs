use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};

use crate::{
    FPS,
    randomisation_strategy::{RandomisationStrategy, black_white::BlackWhiteStrategy},
    screen_buffer::ScreenBuffer,
};

pub struct Display {
    screen_buffer: ScreenBuffer,
    noise_strategy: Box<dyn RandomisationStrategy>,
    mask: Option<Box<[bool]>>,
    window: Window,
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

    pub fn run(&mut self) {
        assert!({
            if let Some(mask) = &self.mask {
                self.screen_buffer.width() * self.screen_buffer.height() == mask.iter().len()
            } else {
                true
            }
        });

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
