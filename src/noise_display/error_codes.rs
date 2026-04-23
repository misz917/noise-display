#[derive(Debug)]
pub enum NoiseDisplayError {
    FailedWindowInitialization(minifb::Error),
    FailedWindowUpdate,
    Unexpected,
}

impl From<minifb::Error> for NoiseDisplayError {
    fn from(value: minifb::Error) -> Self {
        NoiseDisplayError::FailedWindowInitialization(value)
    }
}
