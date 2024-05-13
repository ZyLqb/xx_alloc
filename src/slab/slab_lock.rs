use super::slab_allocator::SlabAllocator;
use core::alloc::{GlobalAlloc, Layout};
use spin::Mutex;

pub struct LockedSlab(Mutex<SlabAllocator>);

unsafe impl Send for LockedSlab {}

impl LockedSlab {
    pub const fn new_uninit() -> Self {
        LockedSlab(Mutex::new(SlabAllocator::new()))
    }
    pub fn init(&self, bottom: usize, top: usize) {
        unsafe { self.0.lock().init(bottom, top) };
    }

    // pub fn allocate_fit(&self, layout: Layout){
    //     self.0.lock().allocate_fit(layout).unwrap()
    // }
}

unsafe impl GlobalAlloc for LockedSlab {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.lock().allocate_fit(layout).expect("alloc err")
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().deallocate_fit(ptr, layout)
    }
}
