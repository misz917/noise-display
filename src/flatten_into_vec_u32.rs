use image::{DynamicImage, GenericImageView};

pub trait FlattenIntoVec<U32> {
    fn flatten(&self) -> Vec<u32>;
}

impl FlattenIntoVec<u32> for DynamicImage {
    fn flatten(&self) -> Vec<u32> {
        let mut output = Vec::new();
        for (_, _, pixel) in self.pixels() {
            let r = pixel.0[0] as u32;
            let g = pixel.0[1] as u32;
            let b = pixel.0[2] as u32;
            let pixel = r << 16 | g << 8 | b;
            output.push(pixel);
        }

        return output;
    }
}
