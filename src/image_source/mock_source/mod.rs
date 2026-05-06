use crate::image_source::{HasStaticDimensions, ImageSource};

#[derive(Default)]
pub struct MockSource;

impl ImageSource for MockSource {
    fn new(_path: &std::path::Path) -> Result<Self, super::error_codes::ImageSourceError>
    where
        Self: Sized,
    {
        Ok(MockSource)
    }

    fn next(&mut self) -> Option<super::indexed_image::IndexedImage> {
        None
    }

    fn fps(&self) -> usize {
        30
    }
}

impl HasStaticDimensions for MockSource {
    fn width(&self) -> usize {
        1280
    }

    fn height(&self) -> usize {
        720
    }
}

