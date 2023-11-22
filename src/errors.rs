
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    TypstEcoStringError(typst_library::prelude::EcoString),
}
