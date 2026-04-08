use crate::{image_source::ImageSource, noise_strategy::NoiseStrategy};

pub trait NoiseDisplayInterface {
    fn new(
        target_fps: usize,
        noise_strategy: Box<dyn NoiseStrategy>,
        binarization_threshold: u8,
    ) -> Self
    where
        Self: Sized;

    fn display(&mut self, image_source: Box<dyn ImageSource>);
}
