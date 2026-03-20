use image::DynamicImage;
use std::path::Path;

pub mod dir_image_source;
pub mod single_image_source;

pub trait ImageSource {
    fn new(path: &Path) -> Self
    where
        Self: Sized;
    fn next(&mut self) -> Option<DynamicImage>;
}
