use crate::screen_buffer::ScreenBuffer;

pub mod black_white;
pub mod rainbow;

pub trait RandomisationStrategy {
    fn randomise(&self, buffer: &mut ScreenBuffer, mask: Option<&[bool]>);
}
