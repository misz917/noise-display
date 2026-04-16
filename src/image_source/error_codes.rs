use crate::image_source::{
    jpg_source::error_codes::JpgSourceError, mp4_source::error_codes::Mp4SourceError,
};

#[derive(Debug)]
pub enum ImageSourceError {
    Mp4SourceError(Mp4SourceError),
    JpgSourceError(JpgSourceError),
}

impl From<Mp4SourceError> for ImageSourceError {
    fn from(value: Mp4SourceError) -> Self {
        ImageSourceError::Mp4SourceError(value)
    }
}
