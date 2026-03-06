use crate::{randomisation_strategy::RandomisationStrategy, screen_buffer::ScreenBuffer};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

pub struct RainbowStrategy;

impl RandomisationStrategy for RainbowStrategy {
    fn randomise(buffer: &mut ScreenBuffer) {
        buffer.get_buffer_mut().par_iter_mut().for_each(|pixel| {
            *pixel = rand::random_range(0..=0xFFFFFF);
        });
    }
}
