use crate::image_source::mp4_source::Mp4SourceError;
use image::DynamicImage;
use std::path::Path;

pub mod image_source_factory;

pub mod jpg_source;
pub mod mp4_source;

#[derive(Debug)]
pub enum ImageSourceError {
    Mp4SourceError(Mp4SourceError),
}

pub trait HasStaticDimensions {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub trait ImageSource: HasStaticDimensions {
    fn new(path: &Path) -> Result<Self, ImageSourceError>
    where
        Self: Sized;
    fn next(&mut self) -> Option<DynamicImage>;
}

pub(crate) struct Dimensions {
    width: usize,
    height: usize,
}

impl HasStaticDimensions for Dimensions {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}
