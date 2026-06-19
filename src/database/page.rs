use crate::database::file::FILE_NAME_MAX;
use crate::database::file::File;
pub const PAGE_SIZE: usize = 8000;
pub const MAX_CACHES: usize = 100;
pub const MAX_OPENED_FILES: usize = 100;

//need to create threadlocal with all the files stored
thread_local! {static OPENED_FILES: [Option<File>;MAX_OPENED_FILES] = [const{None}; MAX_OPENED_FILES];}

//need to create threadlocal with  the cache
thread_local! {static CACHES: [Option<Page>; MAX_CACHES]= [const{None}; MAX_CACHES];}

//have unsafe cell that can give out mutiple refferences to the same thing at the same time

pub struct Page {
    buffer: [u8; PAGE_SIZE],
    file_name: [u8; FILE_NAME_MAX],
    index_in_opened_files: usize,
}

impl Page {
    pub fn get_page() {
        unimplemented!()
    }
}
