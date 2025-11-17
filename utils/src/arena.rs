pub mod arena_ref;
use arena_ref::Ar;
use std::cell::UnsafeCell;
use std::mem::{MaybeUninit, align_of, size_of};
use std::ptr::{self};
pub struct Arena {
    inner: UnsafeCell<ArenaInner>,
}
struct ArenaInner {
    current_buf: Box<[MaybeUninit<u8>]>,
    old_bufs: Vec<Box<[MaybeUninit<u8>]>>,
    offset: usize,
}
impl Arena {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }
    pub fn with_capacity(capacity: usize) -> Self {
        let buf = Box::new_uninit_slice(capacity);
        Self {
            inner: UnsafeCell::new(ArenaInner {
                current_buf: buf,
                old_bufs: Vec::new(),
                offset: 0,
            }),
        }
    }
    pub fn alloc<T>(&self, value: T) -> Ar<'_, T> {
        let inner = unsafe { &mut *self.inner.get() };
        let align = align_of::<T>();
        let size = size_of::<T>();
        let padding = (align - inner.offset % align) % align;
        let new_offset = inner.offset + padding + size;
        if new_offset > inner.current_buf.len() {
            let new_capacity = inner.current_buf.len() * 2;
            let new_capacity = new_capacity.max(size);
            let new_buf: Box<[MaybeUninit<u8>]> = Box::new_uninit_slice(new_capacity);
            let old_buf = std::mem::replace(&mut inner.current_buf, new_buf);
            inner.old_bufs.push(old_buf);
        }
        let start = inner.offset + padding;
        let ptr = unsafe { inner.current_buf.as_mut_ptr().add(start) as *mut T };
        unsafe {
            ptr::write(ptr, value);
        }
        inner.offset = start + size;
        Ar::new(ptr)
    }
}
impl Default for Arena {
    fn default() -> Self {
        Arena::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiple_inserts() {
        let arena = Arena::new();
        let a = arena.alloc(123);
        let b = arena.alloc(456);
        let c = arena.alloc(true);
        assert_eq!(*a, 123);
        assert_eq!(*b, 456);
        assert_eq!(*c, true);
    }
    #[test]
    fn test_ref_eq() {
        let arena = Arena::new();
        let a = arena.alloc("apple".to_string());
        let b = arena.alloc("apple".to_string());
        assert_eq!(a, b);
    }
    #[test]
    fn test_ref_clone() {
        let arena = Arena::new();
        let a = arena.alloc("apple".to_string());
        let b = a.clone();
        assert_eq!(a, b);
    }
    #[test]
    fn test_storing_refs() {
        let arena = Arena::new();
        let a = arena.alloc(123);
        let mut my_refs = vec![];
        my_refs.push(a);
        assert_eq!(*my_refs[0], 123);
    }
    fn identity(n: Ar<i32>) -> Ar<i32> {
        n
    }
    #[test]
    fn test_copy_ref() {
        let arena = Arena::new();
        let n = arena.alloc(123);
        assert_eq!(n, identity(n));
    }
}