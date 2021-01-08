#[derive(Debug)]
pub enum SdlError {
    UnsupportedFeature { msg: String },
    OutOfMemory,
}

pub type Result<T> = std::result::Result<T, SdlError>;
