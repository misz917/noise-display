use image::DynamicImage;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::binarization_strategy::BinarizationStrategy;

pub struct BasicBinarizationStrategy {
    threshold: u8,
}

impl BasicBinarizationStrategy {
    pub fn new() -> Self {
        todo!()
    }
}

impl BinarizationStrategy for BasicBinarizationStrategy {
    fn binarize(&self, image: DynamicImage) -> Vec<bool> {
        let gray = image.to_luma8();

        gray.par_iter()
            .map(|pixel| *pixel > self.threshold)
            .collect()
    }
}
