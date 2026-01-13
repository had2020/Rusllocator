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
