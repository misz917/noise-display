#[derive(Debug)]
pub enum Mp4SourceError {
    FailedToCreateTemporaryDirectory,
    NoImageRead,
    FailedToReadTemporaryDirectory,
    FfmpegFrameExtractionError,
}
