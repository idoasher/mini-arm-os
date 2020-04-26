extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use cortex_m::{interrupt, interrupt::Mutex};

extern "C" {
    fn malloc(size: cty::c_uint) -> Option<NonNull<cty::c_void>>;
    fn free(pointer: *mut cty::c_void);
}

struct InnerCAllocator {}

impl InnerCAllocator {
    fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() == 0 {
            return 0 as *mut u8;
        }
        unsafe {
            match malloc(layout.size() as cty::c_uint) {
                None => 0 as *mut u8,
                Some(address) => address.as_ptr() as *mut u8,
            }
        }
    }

    fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe {
            free(ptr as *mut cty::c_void);
        }
    }
}

pub struct MiniAllocator {
    access: Mutex<InnerCAllocator>,
}

impl MiniAllocator {
    pub const fn empty() -> MiniAllocator {
        MiniAllocator {
            access: Mutex::new(InnerCAllocator {}),
        }
    }
}

unsafe impl GlobalAlloc for MiniAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        interrupt::free(|cs| self.access.borrow(cs).alloc(layout))
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        interrupt::free(|cs| self.access.borrow(cs).dealloc(ptr, layout))
    }
}

#[alloc_error_handler]
pub fn oom_panic(_layout: Layout) -> ! {
    panic!("Out Of Memory");
}
