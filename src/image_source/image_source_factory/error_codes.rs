#[derive(Debug)]
pub enum ImageSourceFactoryError {
    UnsupportedFileExtension(String),
}
