use core::{alloc::{Layout, GlobalAlloc}, ptr::NonNull};

use xx_mutex_lock::Mutex;

use crate::linklist::{link::Linkedlist, def::PGSZ};

pub struct Heap {
    used: usize,
    free_list: Linkedlist,
}

unsafe impl Send for Heap{}

impl Heap {
    pub const fn new_uninit() -> Self{
        Self { used: 0, free_list: Linkedlist::new() }
    }

    pub unsafe fn init(&mut self,bottom: usize,top: usize){
        self.used = 0;
        self.free_list.init(bottom, top - bottom)
    }

    pub unsafe fn allocate_fit(&mut self, layout: Layout) -> Result<*mut u8,Layout>{
        let size = PGSZ;
        self.used += PGSZ;
        Ok(self.free_list.alloc())
    }

    pub unsafe fn deallicate(&mut self,ptr: * mut u8,layout: Layout){
        self.used -= PGSZ;
        self.free_list.dealloc(ptr as usize)
    }
}



pub struct LockedHeap(Mutex<Heap>);

unsafe impl Send for LockedHeap{}

impl LockedHeap {
    pub const fn new() -> Self{
        let heap = Heap::new_uninit();
        LockedHeap(Mutex::new(heap))
    }

    pub fn init(&self,bottom: usize,top:usize){
        unsafe { self.0.lock().init(bottom, top) };
    }
}

unsafe impl  GlobalAlloc for LockedHeap {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    self.0.lock().allocate_fit(layout).unwrap()
  }  
  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
      self.0.lock().deallicate(ptr, layout)
  }
} 