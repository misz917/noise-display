use crate::screen_buffer::ScreenBuffer;

pub mod black_white;
pub mod rainbow;

pub(crate) trait RandomisationStrategy {
    fn randomise(buffer: &mut ScreenBuffer, mask: Option<&[bool]>);
}
