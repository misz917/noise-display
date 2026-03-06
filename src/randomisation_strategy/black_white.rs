use crate::{
    color::{BLACK, WHITE},
    randomisation_strategy::RandomisationStrategy,
    screen_buffer::ScreenBuffer,
};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub struct BlackWhiteStrategy;

impl RandomisationStrategy for BlackWhiteStrategy {
    /// mask: if true => change the pixel; if false => leave unchanged
    fn randomise(buffer: &mut ScreenBuffer, mask: Option<&[bool]>) {
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
                    if rand::random_bool(0.5) {
                        *pixel = BLACK;
                    } else {
                        *pixel = WHITE;
                    }
                }
            });
    }
}
