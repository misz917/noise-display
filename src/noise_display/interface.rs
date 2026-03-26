use crate::{image_source::ImageSource, randomisation_strategy::RandomisationStrategy};

pub trait NoiseDisplayInterface {
    fn new(target_fps: usize, noise_strategy: Box<dyn RandomisationStrategy>) -> Self
    where
        Self: Sized;

    fn display(&mut self, image_source: Box<dyn ImageSource>);
}
