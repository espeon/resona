use thiserror::Error;

#[derive(Error, Debug)]
pub enum SonaError {
    #[error("no patterns available (all weights are zero)")]
    NoPatterns,

    #[error("minimum length must be at least 1")]
    MinLengthTooSmall,

    #[error("minimum length ({0}) cannot exceed maximum length ({1})")]
    InvalidLengthRange(usize, usize),
}
