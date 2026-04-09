use crate::image_source::{
    jpg_source::error_codes::JpgSourceError, mp4_source::error_codes::Mp4SourceError,
};

#[derive(Debug)]
pub enum ImageSourceError {
    Mp4SourceError(Mp4SourceError),
    JpgSourceError(JpgSourceError),
}
