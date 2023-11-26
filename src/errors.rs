#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("{0}")]
    TeraError(#[from] tera::Error),

    #[error("{0}")]
    TracingSubscriberParseError(#[from] tracing_subscriber::filter::ParseError),

    #[error("{0}")]
    TypstEcoStringError(typst_library::prelude::EcoString),
}
