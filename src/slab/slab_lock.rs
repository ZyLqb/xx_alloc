use super::slab::SlabAllocator;
use core::alloc::{GlobalAlloc, Layout};
use xx_mutex_lock::Mutex;

pub struct LockedSlab(Mutex<SlabAllocator>);

unsafe impl Send for LockedSlab {}

impl LockedSlab {
    pub const fn new() -> Self {
        let heap = SlabAllocator::new_uninit();
        LockedSlab(Mutex::new(heap))
    }
    pub fn init(&self, bottom: usize, top: usize) {
        unsafe { self.0.lock().init(bottom, top) };
    }
}

unsafe impl GlobalAlloc for LockedSlab {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.lock().allocate_fit(layout).unwrap()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().deallocate_fit(ptr, layout)
    }
}
