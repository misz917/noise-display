#[derive(Debug)]
pub enum Mp4SourceError {
    FailedToCreateTemporaryDirectory,
    NoImageRead,
    FailedToReadTemporaryDirectory(std::io::Error),
    FfmpegFrameExtractionError,
    FailedTemporaryDirCleanup(std::io::Error),
}

impl std::fmt::Display for Mp4SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
