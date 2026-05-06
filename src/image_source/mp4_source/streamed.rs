use crate::{
    extract_frames::{extract_frames_with_ffmpeg, get_fps},
    image_source::{
        Dimensions, HasStaticDimensions, ImageSource, error_codes::ImageSourceError,
        indexed_image::IndexedImage, mp4_source::error_codes::Mp4SourceError,
    },
};
use std::{
    collections::VecDeque,
    env, fs,
    path::{Path, PathBuf},
};
use tracing::{error, info};
use uuid::Uuid;

pub(crate) struct Streamed {
    input_path: Box<PathBuf>,
    dimensions: Dimensions,
    temp_dir_path: Option<PathBuf>,
    image_paths: VecDeque<(usize, PathBuf)>,
}

impl ImageSource for Streamed {
    fn new(path: &std::path::Path) -> Result<Self, ImageSourceError>
    where
        Self: Sized,
    {
        let mut temp_dir_path = env::temp_dir();
        let uuid = Uuid::new_v4();
        let uuid_path = PathBuf::from(&uuid.to_string());

        temp_dir_path.push(uuid_path);
        info!("Temporary directory path {:?}", temp_dir_path);

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

        let mut memory: VecDeque<(usize, PathBuf)> = VecDeque::new();
        let paths = fs::read_dir(&temp_dir_path)
            .map_err(Mp4SourceError::FailedToReadTemporaryDirectory)?;
        for (i, file_name) in paths.map(|f| f.unwrap().file_name()).enumerate() {
            let full_path = &temp_dir_path.join(file_name);
            memory.push_back((i, full_path.to_owned()));
        }

        let first_image_path = &memory[0].1;
        let first_image = image::open(first_image_path)?;
        let dimensions = Dimensions {
            width: first_image.width() as usize,
            height: first_image.height() as usize,
        };

        Ok(Self {
            dimensions,
            input_path: Box::new(path.to_owned()),
            temp_dir_path: Some(temp_dir_path),
            image_paths: memory,
        })
    }

    fn next(&mut self) -> Option<IndexedImage> {
        let path = self.image_paths.pop_back();
        if let Some((index, path)) = path {
            let image = image::open(&path).ok()?;
            let indexed_image = IndexedImage::new(index, image);
            Some(indexed_image)
        } else {
            None
        }
    }

    fn fps(&self) -> usize {
        get_fps(&self.input_path).unwrap() as usize
    }
}

impl HasStaticDimensions for Streamed {
    fn width(&self) -> usize {
        self.dimensions.width()
    }

    fn height(&self) -> usize {
        self.dimensions.height()
    }
}

impl Streamed {
    fn remove_temporary_dir(path: &Path) -> Result<(), Mp4SourceError> {
        fs::remove_dir_all(path).map_err(Mp4SourceError::FailedTemporaryDirCleanup)
    }
}

impl Drop for Streamed {
    fn drop(&mut self) {
        if let Some(path) = &self.temp_dir_path
            && let Err(err) = Self::remove_temporary_dir(path) {
                error!("{}", err);
            }
    }
}
