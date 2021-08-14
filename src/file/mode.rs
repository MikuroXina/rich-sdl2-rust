use std::ffi::CString;

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
    pub fn new() -> Self {
        Default::default()
    }

    pub fn read(&mut self, read: bool) -> &mut Self {
        self.read = read;
        self
    }
    pub fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }
    pub fn append(&mut self, append: bool) -> &mut Self {
        self.append = append;
        if self.write {
            self.write = false;
        }
        self
    }
    pub fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        if self.append {
            self.append = false;
        }
        self
    }
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        self
    }
    pub fn binary(&mut self, binary: bool) -> &mut Self {
        self.binary = binary;
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
