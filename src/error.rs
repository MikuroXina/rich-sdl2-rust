use static_assertions::assert_impl_all;

/// An error occurred from SDL2.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SdlError {
    /// The feature is unsupported on the platform.
    UnsupportedFeature,
    /// There is no free memory.
    OutOfMemory,
    /// There is other reasons.
    Others {
        /// The message describing reasons.
        msg: String,
    },
}

assert_impl_all!(SdlError: Send, Sync);

/// A specialized [`std::result::Result`] type for this crate.
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
