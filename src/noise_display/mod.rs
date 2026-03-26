use crate::{
    BINARIZATION_THRESHOLD, DEFAULT_TARGET_FPS,
    image_source::ImageSource,
    into_binary::IntoFlatBinary,
    noise_display::interface::NoiseDisplayInterface,
    randomisation_strategy::{RandomisationStrategy, black_white::BlackWhiteStrategy},
    screen_buffer::ScreenBuffer,
};
use minifb::{Key, Scale, Window, WindowOptions};

pub mod interface;

const WINDOW_NAME: &str = "Noise Display";
const DEFAULT_BINARIZATION_THRESHOLD: u8 = 127;

pub struct NoiseDisplay {
    target_fps: usize,
    binarization_threshold: u8,
    noise_strategy: Box<dyn RandomisationStrategy>,
}

impl NoiseDisplayInterface for NoiseDisplay {
    fn new(
        target_fps: usize,
        noise_strategy: Box<dyn RandomisationStrategy>,
        binarization_threshold: u8,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            target_fps,
            binarization_threshold,
            noise_strategy,
        }
    }

    fn display(&mut self, mut image_source: Box<dyn ImageSource>) {
        // first time it has to return a Some
        let mut image = image_source.next().unwrap();

        let width = image.width() as usize;
        let height = image.height() as usize;

        let window = self.init_window(width, height, self.target_fps).unwrap();
        let mut scren_buffer = ScreenBuffer::new(width, height);

        let mut mask = image.binarize_and_flatten(self.binarization_threshold);

        while window.is_open() && !window.is_key_down(Key::Escape) {
            self.noise_strategy
                .randomise(&mut scren_buffer, Some(&mask));

            if let Some(new_image) = image_source.next() {
                image = new_image;
                mask = image.binarize_and_flatten(self.binarization_threshold);
            }
        }
    }
}

impl Default for NoiseDisplay {
    fn default() -> Self {
        Self {
            target_fps: DEFAULT_TARGET_FPS,
            binarization_threshold: DEFAULT_BINARIZATION_THRESHOLD,
            noise_strategy: Box::new(BlackWhiteStrategy),
        }
    }
}

impl NoiseDisplay {
    fn init_window(
        &mut self,
        width: usize,
        height: usize,
        target_fps: usize,
    ) -> Result<Window, Box<dyn std::error::Error>> {
        let mut window = Window::new(
            WINDOW_NAME,
            width,
            height,
            WindowOptions {
                scale: Scale::X4,
                ..WindowOptions::default()
            },
        )?;
        window.set_target_fps(target_fps);

        Ok(window)
    }
}
