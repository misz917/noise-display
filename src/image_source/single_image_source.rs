use crate::image_source::ImageSource;
use image::DynamicImage;

pub struct SingleImageSource {
    image: DynamicImage,
}

impl ImageSource for SingleImageSource {
    fn new(path: &std::path::Path) -> Self {
        assert!(path.is_file());

        let image = image::open(path).unwrap();

        Self { image }
    }

    fn next(&mut self) -> Option<image::DynamicImage> {
        return Some(self.image.clone());
    }
}
