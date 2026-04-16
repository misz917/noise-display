use image::ImageError;

#[derive(Debug)]
pub enum Mp4SourceError {
    FailedToCreateTemporaryDirectory,
    ImageError(ImageError),
}
