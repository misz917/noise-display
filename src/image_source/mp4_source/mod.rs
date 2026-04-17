use crate::{
    extract_frames::extract_frames_with_ffmpeg,
    image_source::{
        Dimensions, HasStaticDimensions, ImageSource, ImageSourceError,
        mp4_source::error_codes::Mp4SourceError,
    },
};
use image::DynamicImage;
use std::{
    collections::LinkedList,
    env, fs,
    path::{Path, PathBuf},
};
use tracing::error;
use uuid::Uuid;

pub mod error_codes;

pub(crate) struct Mp4Source {
    dimensions: Dimensions,
    memory: LinkedList<DynamicImage>,
    temp_dir_path: Option<PathBuf>,
}

impl ImageSource for Mp4Source {
    fn new(path: &std::path::Path) -> Result<Mp4Source, ImageSourceError>
    where
        Self: Sized,
    {
        let mut temp_dir_path = env::temp_dir();
        let uuid = Uuid::new_v4();
        let uuid_path = PathBuf::from(&uuid.to_string());

        temp_dir_path.push(uuid_path);

        if let Err(err) = fs::create_dir_all(&temp_dir_path) {
            match err.kind() {
                std::io::ErrorKind::AlreadyExists => { /* nothing */ }
                _ => {
                    return Err(ImageSourceError::Mp4SourceError(
                        Mp4SourceError::FailedToCreateTemporaryDirectory,
                    ));
                }
            }
        }

        if let Err(_err) = extract_frames_with_ffmpeg(path, &temp_dir_path)
            .map_err(|_| Mp4SourceError::FfmpegFrameExtractionError)
        {
            Self::remove_temporary_dir(&temp_dir_path)?;
        }

        let mut memory = LinkedList::new();
        let paths = fs::read_dir(&temp_dir_path)
            .map_err(|err| Mp4SourceError::FailedToReadTemporaryDirectory(err))?;
        for (i, file_name) in paths.map(|f| f.unwrap().file_name()).enumerate() {
            let image = image::open(path.join(file_name))?;
            memory.push_back(image);
            print!("\r{}", i);
        }

        let first_image = memory.front().ok_or(Mp4SourceError::NoImageRead)?;
        let width = first_image.width().try_into()?;
        let height = first_image.height().try_into()?;

        let dimensions = Dimensions { width, height };

        Ok(Self {
            dimensions,
            memory,
            temp_dir_path: Some(temp_dir_path),
        })
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
        self.dimensions.height()
    }
}

impl Mp4Source {
    fn remove_temporary_dir(path: &Path) -> Result<(), Mp4SourceError> {
        fs::remove_dir_all(path).map_err(|err| Mp4SourceError::FailedTemporaryDirCleanup(err))
    }
}

impl Drop for Mp4Source {
    fn drop(&mut self) {
        if let Some(path) = &self.temp_dir_path {
            if let Err(err) = Self::remove_temporary_dir(path) {
                error!("{}", err);
            }
        }
    }
}
