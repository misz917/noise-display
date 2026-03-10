use crate::{
    color::{BLACK, WHITE},
    randomisation_strategy::RandomisationStrategy,
    screen_buffer::ScreenBuffer,
};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub struct BlackWhiteStrategy;

impl RandomisationStrategy for BlackWhiteStrategy {
    /// mask: if true => change the pixel; if false => leave unchanged
    fn randomise(&self, buffer: &mut ScreenBuffer, mask: Option<&[bool]>) {
        buffer
            .get_buffer_mut()
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pixel)| {
                if mask.is_some() && mask.unwrap()[i] {
                    if rand::random_bool(0.5) {
                        *pixel = BLACK;
                    } else {
                        *pixel = WHITE;
                    }
                }
            });
    }
}
