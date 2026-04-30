use crate::image_source::{
    ImageSource,
    image_source_factory::error_codes::ImageSourceFactoryError,
    jpg_source::JpgSource,
    mp4_source::{self},
};
use std::path::Path;

pub mod error_codes;

pub struct ImageSourceFactory;

impl ImageSourceFactory {
    pub fn new_image_source(path: &Path) -> Result<Box<dyn ImageSource>, ImageSourceFactoryError> {
        let extension = path
            .extension()
            .ok_or(ImageSourceFactoryError::UnsupportedFileExtension(
                path.to_str().unwrap().to_string(),
            ))?
            .to_str()
            .ok_or(ImageSourceFactoryError::NonUtf8Extension)?;

        let image_source: Box<dyn ImageSource> = match extension {
            "jpg" => Box::new(JpgSource::new(path).unwrap()),
            "mp4" => Box::new(mp4_source::streamed::Streamed::new(path).unwrap()),
            _ => {
                return Err(ImageSourceFactoryError::UnsupportedFileExtension(
                    extension.to_string(),
                ));
            }
        };

        Ok(image_source)
    }
}
