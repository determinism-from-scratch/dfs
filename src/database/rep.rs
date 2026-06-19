use crate::abstraction::{
    environment::Environment,
    file_system::{File, FileSystem},
};

use core::{char::MAX, iter::Iterator};
use std::{cell::UnsafeCell, sync::RwLock};

const CACHE_SIZE: usize = 10;
const PAGE_SIZE: usize = 16000;
const MAX_TABLES: usize = 16000;
pub const MAX_PATH: usize = 4096;

#[derive(Clone)]
pub struct CacheInfo {
    name: [u8; MAX_PATH],
    name_len: usize,
    hotness: usize,
}

pub struct Table<FS: FileSystem> {
    file_name: [u8; MAX_PATH],
    name_len: usize,
    file: FS::File,
}

//like ghost in a shell
pub struct Rep<FS: FileSystem> {
    env: Environment<FS>,
    page_cache: [UnsafeCell<[u8; PAGE_SIZE]>; CACHE_SIZE],
    cache_index: [Option<CacheInfo>; CACHE_SIZE],
    tables: [Option<Table<FS>>; MAX_TABLES],
}

impl<FS: FileSystem> Rep<FS> {
    pub fn new(env: Environment<FS>) -> Self {
        Self {
            env,
            page_cache: [const { UnsafeCell::new([0; PAGE_SIZE]) }; CACHE_SIZE],
            cache_index: [const { None }; CACHE_SIZE],
            tables: [const { None }; MAX_TABLES],
        }
    }
}

impl<FS: FileSystem> Rep<FS> {
    //this is my entry point to the program i expect the allocations and io to be inside the
    //environment
    pub fn run(&mut self) -> ! {
        let file = self
            .env
            .file_system
            .open("hello", crate::abstraction::file_system::OpenMode::Read)
            .unwrap();
        unimplemented!()
    }
    pub fn get_free_page(&mut self) -> (Option<CacheInfo>, usize) {
        let indexes = self
            .cache_index
            .iter()
            .enumerate()
            .find(|(i, e)| e.is_none());
        if let Some((i, e)) = indexes {
            return (e.clone(), i);
        }
        unimplemented!();
    }
}
