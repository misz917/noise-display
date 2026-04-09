#[derive(Debug)]
pub enum ImageSourceFactoryError {
    UnsupportedFileExtension(String),
    MissingFileExtension(String),
    NonUtf8Extension,
}
