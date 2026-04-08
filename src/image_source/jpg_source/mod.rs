use crate::image_source::{
    Dimensions, HasStaticDimensions, ImageSource, ImageSourceError,
    jpg_source::error::JpgSourceError,
};
use image::DynamicImage;

pub mod error;

pub(crate) struct JpgSource {
    dimensions: Dimensions,
    image: Option<DynamicImage>,
}

impl ImageSource for JpgSource {
    fn new(path: &std::path::Path) -> Result<JpgSource, ImageSourceError>
    where
        Self: Sized,
    {
        assert!(path.is_file());

        let image = image::open(path).unwrap();
        let dimensions = Dimensions {
            width: image.width().try_into().unwrap(),
            height: image.height().try_into().unwrap(),
        };

        Ok(Self {
            dimensions,
            image: Some(image),
        })
    }

    fn next(&mut self) -> Option<DynamicImage> {
        self.image.take()
    }
}

impl HasStaticDimensions for JpgSource {
    fn width(&self) -> usize {
        self.dimensions.width()
    }

    fn height(&self) -> usize {
        self.dimensions.height()
    }
}
