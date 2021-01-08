#[derive(Debug)]
pub enum SdlError {
    UnsupportedFeature { msg: String },
    OutOfMemory,
    Others { msg: String },
}

pub type Result<T> = std::result::Result<T, SdlError>;
