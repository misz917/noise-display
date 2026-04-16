use image::ImageError;

use crate::image_source::error_codes::ImageSourceError;

#[derive(Debug)]
pub enum Mp4SourceError {
    FailedToCreateTemporaryDirectory,
    ImageError(ImageError),
    NoImageRead,
    FailedU32UsizeConversion,
}

impl From<std::num::TryFromIntError> for ImageSourceError {
    fn from(_value: std::num::TryFromIntError) -> Self {
        ImageSourceError::Mp4SourceError(Mp4SourceError::FailedU32UsizeConversion)
    }
}
