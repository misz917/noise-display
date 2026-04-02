use crate::image_source::{Dimensions, HasStaticDimensions, ImageSource};
use image::DynamicImage;

pub(crate) struct JpgSource {
    dimensions: Dimensions,
    image: Option<DynamicImage>,
}

impl ImageSource for JpgSource {
    fn new(path: &std::path::Path) -> Self
    where
        Self: Sized,
    {
        assert!(path.is_file());

        let image = image::open(path).unwrap();
        let dimensions = Dimensions {
            width: image.width().try_into().unwrap(),
            height: image.height().try_into().unwrap(),
        };

        Self {
            dimensions,
            image: Some(image),
        }
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
