use image::DynamicImage;
use std::path::Path;

pub mod image_source_factory;

pub mod jpg_source;
pub mod mp4_source;

pub trait HasStaticDimensions {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub trait ImageSource: HasStaticDimensions {
    fn new(path: &Path) -> Self
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
