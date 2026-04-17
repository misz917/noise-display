use crate::image_source::{error_codes::ImageSourceError, indexed_image::IndexedImage};
use std::path::Path;

pub mod error_codes;
pub mod image_source_factory;
pub mod indexed_image;
pub mod jpg_source;
pub mod mp4_source;

pub trait HasStaticDimensions {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub trait ImageSource: HasStaticDimensions {
    fn new(path: &Path) -> Result<Self, ImageSourceError>
    where
        Self: Sized;
    fn next(&mut self) -> Option<IndexedImage>;
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
