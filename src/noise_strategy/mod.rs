use crate::{noise_strategy::black_white::BlackWhiteStrategy, screen_buffer::ScreenBuffer};

pub mod black_white;
pub mod color;
pub mod rainbow;
pub mod slide;

pub trait NoiseStrategy {
    fn init(&self, buffer: &mut ScreenBuffer);
    fn randomise(&self, buffer: &mut ScreenBuffer, mask: Option<&[bool]>);
}

impl Default for Box<dyn NoiseStrategy> {
    fn default() -> Self {
        Box::new(BlackWhiteStrategy)
    }
}
