#[derive(Debug)]
pub enum SdlError {
    UnsupportedFeature,
    OutOfMemory,
    Others { msg: String },
}

pub type Result<T> = std::result::Result<T, SdlError>;
