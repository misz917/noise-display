use image::DynamicImage;

pub struct IndexedImage {
    index: usize,
    image: Option<DynamicImage>,
}

impl IndexedImage {
    pub fn new(index: usize, image: DynamicImage) -> Self {
        Self {
            index,
            image: Some(image),
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn image_peek(&self) -> Option<&DynamicImage> {
        if let Some(image) = &self.image {
            return Some(image);
        }
        return None;
    }

    pub fn image_pop(&mut self) -> Option<DynamicImage> {
        self.image.take()
    }
}
