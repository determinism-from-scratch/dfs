use crate::abstraction::file_system;
use crate::database::page;
use crate::database::page::PAGE_SIZE;

pub const FILE_NAME_MAX: usize = 100;

use std::io;

pub struct File {
    inner: file_system::File,
    file_name: [u8; 100],
}

impl File {
    pub fn get_page(&mut self, index: usize, buf: &mut [u8; PAGE_SIZE]) -> io::Result<u64> {
        self.inner
            .lseek(io::SeekFrom::Start((index * PAGE_SIZE) as u64))?;
        self.inner.read(buf)?;
        return Ok(0);
    }

    pub fn write_page(&mut self, index: usize, buf: &mut [u8; PAGE_SIZE]) -> io::Result<u64> {
        self.inner
            .lseek(io::SeekFrom::Start((index * PAGE_SIZE) as u64))?;
        self.inner.write(buf)?;
        return Ok(0);
    }
}
