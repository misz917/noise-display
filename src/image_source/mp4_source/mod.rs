use crate::{
    extract_frames::extract_frames_with_ffmpeg,
    image_source::{
        Dimensions, HasStaticDimensions, ImageSource, ImageSourceError,
        mp4_source::error::Mp4SourceError,
    },
};
use image::DynamicImage;
use std::{collections::LinkedList, fs, path::PathBuf, str::FromStr};

pub mod error;

const TEMP_FILE_PATH: &str = "./temp/";

pub(crate) struct Mp4Source {
    dimensions: Dimensions,
    memory: LinkedList<DynamicImage>,
}

impl ImageSource for Mp4Source {
    fn new(path: &std::path::Path) -> Result<Mp4Source, ImageSourceError>
    where
        Self: Sized,
    {
        assert!(path.is_file());

        let temp_file_path = PathBuf::from_str(TEMP_FILE_PATH).unwrap();

        if let Err(err) = fs::create_dir(&temp_file_path) {
            match err.kind() {
                std::io::ErrorKind::AlreadyExists => (/* do nothing */),
                _ => {
                    return Err(ImageSourceError::Mp4SourceError(
                        Mp4SourceError::TemporaryDirectoryCreation,
                    ));
                }
            }
        }
        extract_frames_with_ffmpeg(&path, &temp_file_path).unwrap();

        assert!(temp_file_path.is_dir());

        let mut memory = LinkedList::new();
        let paths = fs::read_dir(path).unwrap();
        for (i, file_name) in paths.map(|f| f.unwrap().file_name()).enumerate() {
            let image = image::open(path.join(file_name)).unwrap();
            memory.push_back(image);
            print!("\r{}", i);
        }

        let first_image = memory.front().unwrap();
        let width = first_image.width().try_into().unwrap();
        let height = first_image.height().try_into().unwrap();

        let dimensions = Dimensions { width, height };

        Ok(Self { dimensions, memory })
    }

    fn next(&mut self) -> Option<image::DynamicImage> {
        self.memory.pop_front()
    }
}

impl HasStaticDimensions for Mp4Source {
    fn width(&self) -> usize {
        self.dimensions.width()
    }

    fn height(&self) -> usize {
        self.dimensions.width()
    }
}
