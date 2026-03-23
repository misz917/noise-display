use crate::image_source::ImageSource;

pub trait NoiseDisplayInterface {
    fn new() -> Self
    where
        Self: Sized;

    fn display(&mut self, image_source: Box<dyn ImageSource>);
}
