use crate::{randomisation_strategy::RandomisationStrategy, screen_buffer::ScreenBuffer};

pub struct Noise<T: RandomisationStrategy> {
    rand_strat: T,
}

impl<T: RandomisationStrategy> Noise<T> {
    pub fn new(rand_strat: T) -> Self {
        Self { rand_strat }
    }

    pub fn noise(&self, screen_buffer: &mut ScreenBuffer, mask: Option<&[bool]>) {
        self.rand_strat.randomise(screen_buffer, mask);
    }
}
