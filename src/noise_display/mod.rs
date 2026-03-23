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

    fn display(&mut self, mut image_source: Box<dyn ImageSource>) {
        let image = image_source.next();
    }
}
