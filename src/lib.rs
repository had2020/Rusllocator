#![no_std]

#[repr(C)] 
pub struct BitMapAllocator {
    pub bitmap: usize,
}

impl BitMapAllocator {
    pub fn alloc(&mut self, page_size: usize) -> *mut usize {
        let mut addr: usize = self.bitmap >> self.bitmap.trailing_ones();

        self.bitmap = self.bitmap ^ (1 << addr);

        (addr * page_size) as *mut usize
    } // used for arena allocators so I did not include dealloc
}

/*
/// bitmap where 0 = free and 1 = used.
#[repr(C, align(64))]
pub struct PageBitmap<const PAGE_BYTES: u32> {
    bitmap: u32,
}

impl<const PAGE_BYTES: u32> PageBitmap<PAGE_BYTES> {
    pub fn clear(&mut self) {
        unsafe {
            for i in 0..PAGE_BYTES {
                (i as *mut u32).write(0);
            }
        }
        self.bitmap = 0;
    }

    #[inline(always)]
    pub fn alloc(&mut self) -> Option<u32> {
        let mut addr: Option<u32> = None;
        let mut i: u8 = 1;
        loop {
            // TODO check
            if (self.bitmap << i).trailing_zeros() < (self.bitmap << (i - 1)).trailing_zeros() {
                addr = Some(i as u32 * PAGE_BYTES);
                break;
            }

            i += 1;
        }
        addr
    }

    #[inline(always)]
    pub fn dealloc(page: u32) {
        unsafe {
            for i in 0..PAGE_BYTES {
                ((page + i) as *mut u32).write(0);
            }
        }
    }
}

#[macro_export]
macro_rules! new_pagebitmap {
    ($page_bytes: expr) => {
        const PAGE_BYTES: u32 = $page_bytes;

        PageBitmap {
            bitmap: 0;
        }
    };
}
*/

/*
use core::alloc::{self, GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::ptr::null_mut;
use core::usize;

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
*/
/* Not chache aligned
use core::usize;

#[repr(C, align(64))]
pub struct BitMapAllocator<const PAGES: usize, const PAGE_SIZE_BTYES: usize> {
    bitmap: [u8; PAGES],
}

impl<const PAGES: usize, const PAGE_SIZE_BTYES: usize> BitMapAllocator<PAGES, PAGE_SIZE_BTYES> {
    pub const fn new() -> Self {
        Self { bitmap: [0; PAGES] }
    }

    #[inline(always)]
    pub fn alloc(&mut self) -> usize {
        /*
        let addr: usize;
        let mut i: u8 = 1;
        loop {
            if (self.bitmap << i).trailing_zeros() < (self.bitmap << (i - 1)).trailing_zeros() {
                addr = i as usize * PAGE_BYTES;
                break;
            }

            i += 1;
        }
        addr
        */

        /*
        loop {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                core::arch::asm!()
            }

            #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
            {}

            #[cfg(any(target_arch = "aarch64"))]
            {}
        }
        */
        let addr: usize = 0;
        loop {
            // TODO: use AND and XOR to make this branchless

        }
    }
}
*/
