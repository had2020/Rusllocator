#![no_std]
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::ptr::null_mut;

#[repr(align(64))]
struct Allocator<const BYTES: usize> {
    heap: [u8: BYTES], 
    next: AtomicUsize,
}

impl<const BYTES: usize> Allocator<BYTES> {
    const fn new() -> Self {
        Self {
            heap: [const {0_u8}; BYTES],
            next: AtomicUsize::new(0)
        }
    }
}
