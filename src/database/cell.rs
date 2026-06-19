use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::slice;

static PAGE_SIZE: usize = 8000;

struct CacheCell {
    inner: UnsafeCell<[u8; PAGE_SIZE]>,
}

impl<'a> CacheCell {
    pub fn new() -> Self {
        CacheCell {
            inner: UnsafeCell::new([0; PAGE_SIZE]),
        }
    }
    pub fn get_slice(&'a mut self, start: usize, len: usize) -> &'a [u8] {
        let ptr = self.inner.get_mut();
        let raw_ptr = ptr as *mut u8;
        let slice_ptr = unsafe { raw_ptr.add(start) };
        unsafe { slice::from_raw_parts_mut(slice_ptr, len) }
    }
}
