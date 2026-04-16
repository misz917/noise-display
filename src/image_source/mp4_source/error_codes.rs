#[derive(Debug)]
pub enum Mp4SourceError {
    FailedToCreateTemporaryDirectory,
    NoImageRead,
    FailedToReadTemporaryDirectory(std::io::Error),
    FfmpegFrameExtractionError,
}
