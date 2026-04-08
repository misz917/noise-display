use crate::image_source::{jpg_source::error::JpgSourceError, mp4_source::error::Mp4SourceError};

#[derive(Debug)]
pub enum ImageSourceError {
    Mp4SourceError(Mp4SourceError),
    JpgSourceError(JpgSourceError),
}
