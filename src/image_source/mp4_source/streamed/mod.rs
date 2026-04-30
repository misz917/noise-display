use crate::image_source::{
    HasStaticDimensions, ImageSource, error_codes::ImageSourceError, indexed_image::IndexedImage,
};

pub mod error_codes;

pub(crate) struct Streamed {}

impl ImageSource for Streamed {
    fn new(path: &std::path::Path) -> Result<Self, ImageSourceError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn next(&mut self) -> Option<IndexedImage> {
        todo!()
    }

    fn fps(&self) -> usize {
        todo!()
    }
}

impl HasStaticDimensions for Streamed {
    fn width(&self) -> usize {
        todo!()
    }

    fn height(&self) -> usize {
        todo!()
    }
}
