use minifb::Window;

use crate::{
    randomisation_strategy::{RandomisationStrategy, black_white::BlackWhiteStrategy},
    screen_buffer::ScreenBuffer,
};

pub struct Display {
    screen_buffer: ScreenBuffer,
    noise_strategy: Box<dyn RandomisationStrategy>,
    mask: Option<Box<[bool]>>,
    // window: Window;
}

impl Display {
    pub fn new(screen_buffer: ScreenBuffer) -> Self {
        Self {
            screen_buffer,
            noise_strategy: Box::new(BlackWhiteStrategy),
            mask: None,
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
}

/*
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

    let mut screen_buffer = ScreenBuffer::new(BUFFER_WIDTH, BUFFER_HEIGHT);

    RainbowStrategy::randomise(&mut screen_buffer, None);

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

        RainbowStrategy::randomise(&mut screen_buffer, Some(&mask));

        window
            .update_with_buffer(screen_buffer.get_buffer(), BUFFER_WIDTH, BUFFER_HEIGHT)
            .unwrap();
    }
*/
