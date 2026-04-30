use image::ImageError;

use crate::image_source::mp4_source::memory_preloaded::error_codes::Mp4SourceError;

#[derive(Debug)]
pub enum ImageSourceError {
    Mp4SourceError(Mp4SourceError),
    CommonSourceError(CommonSourceError),
}

#[derive(Debug)]
pub enum CommonSourceError {
    ImageError(ImageError),
    FailedU32UsizeConversion,
}

impl From<Mp4SourceError> for ImageSourceError {
    fn from(value: Mp4SourceError) -> Self {
        ImageSourceError::Mp4SourceError(value)
    }
}

impl From<CommonSourceError> for ImageSourceError {
    fn from(value: CommonSourceError) -> Self {
        ImageSourceError::CommonSourceError(value)
    }
}

impl From<std::num::TryFromIntError> for ImageSourceError {
    fn from(_value: std::num::TryFromIntError) -> Self {
        ImageSourceError::CommonSourceError(CommonSourceError::FailedU32UsizeConversion)
    }
}

impl From<ImageError> for ImageSourceError {
    fn from(value: ImageError) -> Self {
        ImageSourceError::CommonSourceError(CommonSourceError::ImageError(value))
    }
}
