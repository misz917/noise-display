use std::path::Path;

use image::DynamicImage;

pub mod basic_image_source;

pub trait ImageSource {
    fn new(path: &Path) -> Self;
    fn next(&mut self) -> Option<DynamicImage>;
}
