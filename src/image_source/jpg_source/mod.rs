use crate::image_source::{Dimensions, HasStaticDimensions, ImageSource, ImageSourceError};
use image::DynamicImage;

pub(crate) struct JpgSource {
    dimensions: Dimensions,
    image: Option<DynamicImage>,
}

impl ImageSource for JpgSource {
    fn new(path: &std::path::Path) -> Result<JpgSource, ImageSourceError>
    where
        Self: Sized,
    {
        let image = image::open(path)?;
        let dimensions = Dimensions {
            width: image.width().try_into()?,
            height: image.height().try_into()?,
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
