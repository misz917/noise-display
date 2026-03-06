use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    color::{BLACK, WHITE},
    randomisation_strategy::RandomisationStrategy,
    screen_buffer::ScreenBuffer,
};

pub struct BlackWhiteStrategy;

impl RandomisationStrategy for BlackWhiteStrategy {
    fn randomise(buffer: &mut ScreenBuffer) {
        buffer.get_buffer_mut().par_iter_mut().for_each(|pixel| {
            if rand::random_bool(0.5) {
                *pixel = BLACK;
            } else {
                *pixel = WHITE;
            }
        });
    }
}
