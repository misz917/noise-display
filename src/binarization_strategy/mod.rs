use image::DynamicImage;

pub mod basic;

pub trait BinarizationStrategy {
    fn binarize(&self, image: DynamicImage) -> Vec<bool>;
}
