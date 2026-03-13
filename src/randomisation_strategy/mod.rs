use crate::screen_buffer::ScreenBuffer;

pub mod black_white;
pub mod rainbow;
pub mod slide;

pub trait RandomisationStrategy {
    fn init(&self, buffer: &mut ScreenBuffer);
    fn randomise(&self, buffer: &mut ScreenBuffer, mask: Option<&[bool]>);
}
