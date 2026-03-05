use crate::color::{BLACK, WHITE};
use image::DynamicImage;
use rayon::iter::{ParallelBridge, ParallelIterator};

pub trait IntoBinary {
    fn binarize(&mut self, threshold: u8) -> Result<(), ()>;
}

impl IntoBinary for DynamicImage {
    fn binarize(&mut self, threshold: u8) -> Result<(), ()> {
        let mut gray = self.to_luma8();
        let _ = gray.iter_mut().par_bridge().for_each(|pixel| {
            if *pixel > threshold {
                *pixel = WHITE as u8;
            } else {
                *pixel = BLACK as u8;
            }
        });

        *self = DynamicImage::ImageLuma8(gray);
        Ok(())
    }
}
