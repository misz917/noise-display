use crate::{
    display::{DEFAULT_HEIGHT, DEFAULT_WIDTH},
    image_source::ImageSource,
};
use image::{DynamicImage, imageops::FilterType};

pub struct SingleImageSource {
    image: DynamicImage,
}

impl ImageSource for SingleImageSource {
    fn new(path: &std::path::Path) -> Self {
        assert!(path.is_file());

        let image = image::open(path).unwrap();
        let scaled = image.resize_exact(
            DEFAULT_WIDTH.try_into().unwrap(),
            DEFAULT_HEIGHT.try_into().unwrap(),
            FilterType::Nearest,
        );

        Self { image: scaled }
    }

    fn next(&mut self) -> Option<image::DynamicImage> {
        return Some(self.image.clone());
    }
}
