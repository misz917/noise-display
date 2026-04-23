use crate::{
    image_source::ImageSource, noise_display::error_codes::NoiseDisplayError,
    noise_strategy::NoiseStrategy,
};

pub trait NoiseDisplayInterface {
    fn new(
        binarization_threshold: u8,
        noise_strategy: Box<dyn NoiseStrategy>,
        image_source: Option<Box<dyn ImageSource>>,
    ) -> Self
    where
        Self: Sized;

    fn set_image_source(&mut self, image_source: Box<dyn ImageSource>) -> &mut Self;

    fn set_noise_strategy(&mut self, noise_strategy: Box<dyn NoiseStrategy>) -> &mut Self;

    fn run(&mut self) -> Result<(), NoiseDisplayError>;
}
