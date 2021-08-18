//! A file open mode for opening a file conveniently.

use std::ffi::CString;

/// A builder for an open mode of [`super::RwOps`].
#[derive(Debug, Default, Clone)]
pub struct OpenMode {
    read: bool,
    create: bool,
    append: bool,
    write: bool,
    truncate: bool,
    binary: bool,
}

impl OpenMode {
    /// Constructs an empty mode. Please do not pass this into [`super::RwOps`] as is.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets to be able to read.
    pub fn read(&mut self, read: bool) -> &mut Self {
        self.read = read;
        self
    }
    /// Sets to be able to create a new file.
    pub fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }
    /// Sets to force to append. Setting true will disable to be able to write.
    pub fn append(&mut self, append: bool) -> &mut Self {
        self.append = append;
        if self.write {
            self.write = false;
        }
        self
    }
    /// Sets to force to write. Setting true will disable to be able to append.
    pub fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        if self.append {
            self.append = false;
        }
        self
    }
    /// Sets to force to truncate, overwriting all of a file.
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        self
    }
    /// Sets to be able to read as the binary mode. Setting false will be the text mode.
    pub fn binary(&mut self, binary: bool) -> &mut Self {
        self.binary = binary;
        self
    }
    /// Sets to be able to read as the text mode. Setting false will be the binary mode.
    pub fn text(&mut self, text: bool) -> &mut Self {
        self.binary = !text;
        self
    }

    pub(super) fn into_raw(self) -> CString {
        let mut string = if self.read && self.write && self.truncate {
            "w+"
        } else if self.read && self.write {
            "r+"
        } else if self.read && self.append {
            "a+"
        } else if self.write {
            "w"
        } else if self.read {
            "r"
        } else if self.append {
            "a"
        } else {
            panic!("the open mode was empty");
        }
        .to_owned();
        if self.binary {
            string.push('b');
        }
        CString::new(string).unwrap()
    }
}
