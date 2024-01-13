use core::{
    alloc::{GlobalAlloc, Layout},
    ops::{Index, IndexMut},

};
use xxos_log::LOG;

use xx_mutex_lock::Mutex;
use xxos_log::{error, info};

use crate::linklist::{align_to_up, def::*, link::Linkedlist};

pub struct Heap {
    pub used: usize,
    pub pool: [Linkedlist; POLL_COUNT],
}
unsafe impl Send for Heap {}

impl Heap {
    pub const fn new_uninit() -> Self {
        Self {
            used: 0,
            pool: [Linkedlist::new(); POLL_COUNT],
        }
    }
    pub fn align_layout(layout: Layout) -> Result<Layout, ()> {
        fn find_fit_size(size: usize) -> usize {
            match size {
                0 => 0,
                1..33 => POLL_SIZE_32,
                33..65 => POLL_SIZE_64,
                65..129 => POLL_SIZE_128,
                129..257 => POLL_SIZE_256,
                257..513 => POLL_SIZE_512,
                513..1025 => POLL_SIZE_1024,
                1025..2049 => POLL_SIZE_2048,
                2049..4097 => POLL_SIZE_4096,
                _ => align_to_up(size, PGSZ),
            }
        }
        info!("the layout before find  is size {} algin {}",layout.size(),layout.align());
        let fit_size = find_fit_size(layout.size());
        if layout.align() > PGSZ {
            xxos_log::warn!("the Layout align is over 4096")
        }
        info!("the layout is size {} algin {}",fit_size,layout.align());
        Layout::from_size_align(fit_size, layout.align()).map_err(|_| ())
    }

    pub unsafe fn init(&mut self, bottom: usize, top: usize) {
        self.used = 0;
        self.pool
            .index_mut(POLL_PGSZ)
            .init(bottom, top, PGSZ);
    }
    unsafe fn allocate<T>(&mut self, index: usize,size: usize) -> Option<*mut T> {
        info!("allocate the index is {}",index);
        if let Some(ptr) = self.pool.index_mut(index).alloc::<T>() {
            info!("it alloced!");
            Some(ptr)
        } else {
            info!("it go find now page!");
            info!("the size is {}!",size);
            let page = self
                .pool
                .index_mut(POLL_PGSZ)
                .alloc::<T>()
                .expect("None Page");

            let start = page as usize;
            info!("it got a page {}!",page as usize);
            let end = start + PGSZ;
            info!("go dealloc in {}",index);
            for address in (start..end).step_by(size) {
                self.pool.index_mut(index).dealloc(address)
            }
            let ptr = self.pool.index_mut(index).alloc::<T>();
            ptr
        }
    }

    pub unsafe fn allocate_fit(&mut self, layout: Layout) -> Result<*mut u8, ()> {
        info!("allocer start ");
        let fit_layout = Self::align_layout(layout)?;
        let mut ptr = None;
        match fit_layout.size() {
            POLL_SIZE_32 => {
                info!("allocer in 32");
                ptr = self.allocate(POLL_32,POLL_SIZE_32);
            }
            POLL_SIZE_64 => {
                info!("allocer in 64");
                ptr = self.allocate(POLL_64,POLL_SIZE_64);
            }
            POLL_SIZE_128 => {
                ptr = self.allocate(POLL_128,POLL_SIZE_128);
            }
            POLL_SIZE_256 => {
                ptr = self.allocate(POLL_256,POLL_SIZE_256);
            }
            POLL_SIZE_512 => {
                ptr = self.allocate(POLL_512,POLL_SIZE_512);
            }
            POLL_SIZE_1024 => {
                ptr = self.allocate(POLL_1024,POLL_SIZE_1024);
            }
            POLL_SIZE_2048 => {
                ptr = self.allocate(POLL_2048,POLL_SIZE_2048);
            }
            POLL_SIZE_4096 => {
                ptr = self.allocate(POLL_4096,POLL_SIZE_4096);
            }
            _ => {
                info!("allocer start other");
                let size = fit_layout.size();
                if size > self.pool.index(POLL_PGSZ).len() {
                    error!("the alloc size is to big");
                    return Err(());
                }
                //algin to layout
                let algin_size = layout.align();
                loop {
                    let ptr = self
                        .pool
                        .index_mut(POLL_PGSZ)
                        .alloc::<u8>()
                        .expect("None value");
                    if (ptr as usize) / algin_size == 0 {
                        break;
                    } else {
                        //the preformance is prety poor
                        self.pool.index_mut(POLL_PGSZ).dealloc_tail(ptr as usize);
                    }
                }
            }
        };
        self.used += layout.size();
        Ok(ptr.expect("it must have"))
    }

    unsafe fn deallocate(&mut self, index: usize, ptr: *mut u8) {
        self.pool.index_mut(index).dealloc(ptr as usize)
    }

    pub unsafe fn deallocate_fit(&mut self, ptr: *mut u8, layout: Layout) {
        let layout = Self::align_layout(layout).expect("Never run here");
        match layout.size() {
            POLL_SIZE_32 => {
                self.deallocate(POLL_32, ptr);
            }
            POLL_SIZE_64 => {
                self.deallocate(POLL_64, ptr);
            }
            POLL_SIZE_128 => {
                self.deallocate(POLL_128, ptr);
            }
            POLL_SIZE_256 => {
                self.deallocate(POLL_256, ptr);
            }
            POLL_SIZE_512 => {
                self.deallocate(POLL_512, ptr);
            }
            POLL_SIZE_1024 => {
                self.deallocate(POLL_1024, ptr);
            }
            POLL_SIZE_2048 => {
                self.deallocate(POLL_2048, ptr);
            }
            POLL_SIZE_4096 => {
                self.deallocate(POLL_4096, ptr);
            }
            _ => {
                let size = layout.size();
                if size > self.pool.index(POLL_PGSZ).len() {
                    error!("the alloc size is to big");
                }
                self.pool.index_mut(POLL_PGSZ).dealloc(ptr as usize);
            }
        };
        self.used -= layout.size();
    }
}

pub struct LockedHeap(Mutex<Heap>);

unsafe impl Send for LockedHeap {}

impl LockedHeap {
    pub const fn new() -> Self {
        let heap = Heap::new_uninit();
        LockedHeap(Mutex::new(heap))
    }
    pub fn init(&self, bottom: usize, top: usize) {
        unsafe { self.0.lock().init(bottom, top) };
    }
}

unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.lock().allocate_fit(layout).unwrap()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().deallocate_fit(ptr, layout)
    }
}
