use crate::image_source::{ImageSource, jpg_source::JpgSource, mp4_source::Mp4Source};
use std::path::Path;

pub struct ImageSourceFactory;

impl ImageSourceFactory {
    pub fn new_image_source(path: &Path) -> Box<dyn ImageSource> {
        let extension = path.extension().unwrap().to_str().unwrap();
        let image_source: Box<dyn ImageSource> = match extension {
            "jpg" => Box::new(JpgSource::new(path).unwrap()),
            "mp4" => Box::new(Mp4Source::new(path).unwrap()),
            _ => unimplemented!(),
        };

        image_source
    }
}
