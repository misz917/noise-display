use crate::{
    DEFAULT_TARGET_FPS,
    image_source::ImageSource,
    noise_display::interface::NoiseDisplayInterface,
    randomisation_strategy::{RandomisationStrategy, black_white::BlackWhiteStrategy},
};
use minifb::{Scale, Window, WindowOptions};

pub mod interface;

const WINDOW_NAME: &str = "Noise Display";

pub struct NoiseDisplay {
    target_fps: usize,
    window: Option<Window>,
    noise_strategy: Box<dyn RandomisationStrategy>,
}

impl NoiseDisplayInterface for NoiseDisplay {
    fn new(target_fps: usize, noise_strategy: Box<dyn RandomisationStrategy>) -> Self
    where
        Self: Sized,
    {
        Self {
            window: None,
            target_fps,
            noise_strategy,
        }
    }

    fn display(&mut self, mut image_source: Box<dyn ImageSource>) {
        if let Some(image) = image_source.next() {
            let width = image.width() as usize;
            let height = image.height() as usize;
            self.init_window(width, height, self.target_fps).unwrap();
        }
    }
}

impl Default for NoiseDisplay {
    fn default() -> Self {
        Self {
            window: None,
            target_fps: DEFAULT_TARGET_FPS,
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
    ) -> Result<(), Box<dyn std::error::Error>> {
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
        self.window = Some(window);

        Ok(())
    }
}
