use image::DynamicImage;

pub trait IntoFlatBinary {
    fn binarize_and_flatten(&self, threshold: u8) -> Vec<bool>;
}

impl IntoFlatBinary for DynamicImage {
    fn binarize_and_flatten(&self, threshold: u8) -> Vec<bool> {
        let mut output = Vec::new();
        let gray = self.to_luma8();
        gray.iter().for_each(|pixel| {
            if *pixel > threshold {
                output.push(true);
            } else {
                output.push(false);
            }
        });

        output
    }
}
