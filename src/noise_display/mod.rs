use crate::{FPS, image_source::ImageSource, noise_display::interface::NoiseDisplayInterface};
use minifb::{Scale, Window, WindowOptions};

pub mod interface;

const WINDOW_NAME: &str = "Noise Display";

pub struct NoiseDisplay {
    window: Option<Window>,
}

impl NoiseDisplayInterface for NoiseDisplay {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self { window: None }
    }

    fn display(&mut self, mut image_source: Box<dyn ImageSource>) {
        if let Some(image) = image_source.next() {
            let width = image.width() as usize;
            let height = image.height() as usize;
            self.init_window(width, height, FPS).unwrap();
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
