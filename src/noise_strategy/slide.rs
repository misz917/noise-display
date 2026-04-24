use crate::{
    noise_strategy::{
        NoiseStrategy,
        black_white::BlackWhiteStrategy,
        color::{BLACK, WHITE},
    },
    screen_buffer::ScreenBuffer,
};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub struct SlideStrategy;

impl NoiseStrategy for SlideStrategy {
    fn randomise(&self, buffer: &mut ScreenBuffer, mask: Option<&[bool]>) {
        let c_screen_buffer = buffer.clone();
        let c_buffer = c_screen_buffer.get_buffer();
        let width = buffer.width();

        buffer
            .get_buffer_mut()
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pixel)| {
                if mask.is_some() && mask.unwrap()[i] {
                    if i % width != width - 1 {
                        if mask.unwrap()[i + 1] {
                            *pixel = c_buffer[i + 1]
                        } else {
                            if rand::random_bool(0.5) {
                                *pixel = BLACK;
                            } else {
                                *pixel = WHITE;
                            }
                        }
                    } else {
                        if rand::random_bool(0.5) {
                            *pixel = BLACK;
                        } else {
                            *pixel = WHITE;
                        }
                    }
                }
            });
    }

    fn init(&self, buffer: &mut ScreenBuffer) {
        BlackWhiteStrategy.randomise(buffer, None);
    }
}
