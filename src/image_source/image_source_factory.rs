use crate::image_source::{
    ImageSource, dir_image_source::DirImageSource, single_image_source::SingleImageSource,
};
use std::path::Path;

pub struct ImageSourceFactory;

impl ImageSourceFactory {
    pub fn new_image_source(path: &Path) -> Box<dyn ImageSource> {
        let extension = path.extension().unwrap().to_str().unwrap();
        let image_source: Box<dyn ImageSource> = match extension {
            "jpg" => Box::new(SingleImageSource::new(path)),
            "mp4" => Box::new(DirImageSource::new(path)),
            _ => unimplemented!(),
        };

        image_source
    }
}
