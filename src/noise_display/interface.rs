use crate::{
    binarization_strategy::BinarizationStrategy, image_source::ImageSource,
    noise_display::error_codes::NoiseDisplayError, noise_strategy::NoiseStrategy,
};

pub trait NoiseDisplayInterface {
    fn new(
        binarization_threshold: Box<dyn BinarizationStrategy>,
        noise_strategy: Box<dyn NoiseStrategy>,
        image_source: Option<Box<dyn ImageSource>>,
    ) -> Self
    where
        Self: Sized;

    fn run(&mut self) -> Result<(), NoiseDisplayError>;
    fn set_image_source(&mut self, image_source: Box<dyn ImageSource>) -> &mut Self;
    fn set_noise_strategy(&mut self, noise_strategy: Box<dyn NoiseStrategy>) -> &mut Self;
    fn set_binarization_strategy(
        &mut self,
        binarization_strategy: Box<dyn BinarizationStrategy>,
    ) -> &mut Self;
}
