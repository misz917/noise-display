use crate::{randomisation_strategy::RandomisationStrategy, screen_buffer::ScreenBuffer};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub struct RainbowStrategy;

impl RandomisationStrategy for RainbowStrategy {
    /// mask: if true => change the pixel; if false => leave unchanged
    fn randomise(&self, buffer: &mut ScreenBuffer, mask: Option<&[bool]>) {
        let mask = match mask {
            Some(mask) => mask,
            None => &vec![true; buffer.height() * buffer.width()],
        };

        buffer
            .get_buffer_mut()
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pixel)| {
                if mask[i] {
                    *pixel = rand::random_range(0..=0xFFFFFF);
                }
            });
    }
}
