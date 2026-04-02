use crate::image_source::{HasStaticDimensions, ImageSource};

pub(crate) struct Mp4Source;

impl ImageSource for Mp4Source {
    fn new(path: &std::path::Path) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn next(&mut self) -> Option<image::DynamicImage> {
        todo!()
    }
}

impl HasStaticDimensions for Mp4Source {
    fn width(&self) -> usize {
        todo!()
    }

    fn height(&self) -> usize {
        todo!()
    }
}
