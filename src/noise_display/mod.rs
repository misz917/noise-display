use crate::{image_source::ImageSource, noise_display::interface::NoiseDisplayInterface};

pub mod interface;

pub struct NoiseDisplay;

impl NoiseDisplayInterface for NoiseDisplay {
    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn display(&mut self, image_source: Box<dyn ImageSource>) {
        todo!()
    }
}
