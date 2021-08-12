use static_assertions::assert_impl_all;

#[derive(Debug, Clone)]
pub enum SdlError {
    UnsupportedFeature,
    OutOfMemory,
    Others { msg: String },
}

assert_impl_all!(SdlError: Send, Sync);

pub type Result<T> = std::result::Result<T, SdlError>;

impl std::fmt::Display for SdlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SdlError::UnsupportedFeature => f.write_str("the feature is unsupported"),
            SdlError::OutOfMemory => f.write_str("out of memory"),
            SdlError::Others { msg } => f.write_str(msg),
        }
    }
}

impl std::error::Error for SdlError {}
