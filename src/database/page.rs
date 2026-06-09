use crate::abstraction::file_system::{File, FileSystem};
use crate::database::rep::{self, Rep};

pub struct CachedPage<FS: FileSystem> {
    page: Page,
    file: FS::File,
    info: usize,
}

pub struct Page {
    file_name: [u8; rep::MAX_PATH],
    name_len: usize,
    index: usize,
}

impl Page {
    pub fn get_page<FS: FileSystem>(&mut self, rep: &mut Rep<FS>) {
        rep.get_free_page();
    }
}
