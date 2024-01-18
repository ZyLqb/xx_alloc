use core::{
    alloc::Layout,
    ops::{Index, IndexMut},
};
use xxos_log::LOG;
use xxos_log::{error, info};

use crate::{
    align_down, align_up,
    linklist::{def::*, link::Linkedlist},
};

pub struct SlabAllocator {
    pub(crate) pool: [Linkedlist; POOL_COUNT],
}

unsafe impl Send for SlabAllocator {}

impl SlabAllocator {
    pub const fn new_uninit() -> Self {
        Self {
            pool: [Linkedlist::new(); POOL_COUNT],
        }
    }

    pub fn is_pool_empty(&self, index: usize) -> bool {
        !self.pool[index].is_empty()
    }

    pub fn align_layout(layout: Layout) -> Result<Layout, ()> {
        fn find_fit_size(size: usize) -> usize {
            match size {
                0 => 0,
                1..=32 => POOL_SIZE_32,
                33..=64 => POOL_SIZE_64,
                65..=128 => POOL_SIZE_128,
                129..=256 => POOL_SIZE_256,
                257..=512 => POOL_SIZE_512,
                513..=1024 => POOL_SIZE_1024,
                1025..=2048 => POOL_SIZE_2048,
                2049..=4096 => POOL_SIZE_4096,
                _ => align_up!(size, PGSZ),
                // TODO
                // 4096 & _ => use buddy
            }
        }

        info!(
            "the layout before find  is size {} algin {}",
            layout.size(),
            layout.align()
        );

        let fit_size = find_fit_size(layout.size());

        if layout.align() > PGSZ {
            xxos_log::warn!("the Layout align is over 4096")
        }

        info!(
            "the layout is size {:#x} algin {:#x}",
            fit_size,
            layout.align()
        );
        Layout::from_size_align(fit_size, layout.align()).map_err(|_| ())
    }

    pub unsafe fn init(&mut self, bottom: usize, top: usize) {
        self.pool.index_mut(POOL_PGSZ).init(bottom, top, PGSZ);
    }

    unsafe fn allocate<T>(&mut self, index: usize, size: usize) -> Option<*mut T> {
        info!("allocate the index is {}", index);

        if let Some(ptr) = self.pool.index_mut(index).pop::<T>() {
            info!("it alloced!");
            Some(ptr)
        } else {
            // TODO: get new page use buddy
            info!("it go find new page!");
            info!("the size is {:#x}!", size);

            let page = self
                .pool
                .index_mut(POOL_PGSZ)
                .pop::<T>()
                .expect("None Page");

            let start = page as usize;
            info!("it got a page {}!", page as usize);

            let end = start + PGSZ;
            info!("go dealloc in {}", index);

            for address in (start..end).step_by(size) {
                self.pool.index_mut(index).push(address)
            }

            let ptr = self.pool.index_mut(index).pop::<T>();
            ptr
        }
    }

    pub unsafe fn allocate_fit(&mut self, layout: Layout) -> Result<*mut u8, ()> {
        info!("allocate start ");

        let fit_layout = Self::align_layout(layout)?;
        let mut ptr = None;

        match fit_layout.size() {
            POOL_SIZE_32 => {
                info!("allocer in 32");
                ptr = self.allocate(POOL_32, POOL_SIZE_32);
            }
            POOL_SIZE_64 => {
                info!("allocer in 64");
                ptr = self.allocate(POOL_64, POOL_SIZE_64);
            }
            POOL_SIZE_128 => {
                info!("allocer in 128");
                ptr = self.allocate(POOL_128, POOL_SIZE_128);
            }
            POOL_SIZE_256 => {
                info!("allocer in 256");
                ptr = self.allocate(POOL_256, POOL_SIZE_256);
            }
            POOL_SIZE_512 => {
                info!("allocer in 512");
                ptr = self.allocate(POOL_512, POOL_SIZE_512);
            }
            POOL_SIZE_1024 => {
                info!("allocer in 1024");
                ptr = self.allocate(POOL_1024, POOL_SIZE_1024);
            }
            POOL_SIZE_2048 => {
                info!("allocer in 2048");
                ptr = self.allocate(POOL_2048, POOL_SIZE_2048);
            }
            POOL_SIZE_4096 => {
                info!("allocer in 4096");
                ptr = self.allocate(POOL_4096, POOL_SIZE_4096);
            }
            _ => {
                info!("allocer start other");

                let size = fit_layout.size();
                let align_size = layout.align();

                // BUG?
                // is `len` not Linkedlist size?
                if size > self.pool.index(POOL_PGSZ).len() {
                    error!("the alloc size is too big");
                    return Err(());
                }

                // I don't know what's use of layout
                // align to layout
                loop {
                    let ptr = self
                        .pool
                        .index_mut(POOL_PGSZ)
                        .pop::<u8>()
                        .expect("None value");

                    if align_down!(ptr as usize, align_size) == 0 {
                        break;
                    } else {
                        // the preformance is prety poor
                        self.pool.index_mut(POOL_PGSZ).push_tail(ptr as usize);
                    }
                }
            }
        };

        Ok(ptr.expect("it must have"))
    }

    unsafe fn deallocate(&mut self, index: usize, ptr: *mut u8) {
        self.pool.index_mut(index).push(ptr as usize)
    }

    pub unsafe fn deallocate_fit(&mut self, ptr: *mut u8, layout: Layout) {
        let layout = Self::align_layout(layout).expect("Never run here");
        match layout.size() {
            POOL_SIZE_32 => {
                self.deallocate(POOL_32, ptr);
            }
            POOL_SIZE_64 => {
                self.deallocate(POOL_64, ptr);
            }
            POOL_SIZE_128 => {
                self.deallocate(POOL_128, ptr);
            }
            POOL_SIZE_256 => {
                self.deallocate(POOL_256, ptr);
            }
            POOL_SIZE_512 => {
                self.deallocate(POOL_512, ptr);
            }
            POOL_SIZE_1024 => {
                self.deallocate(POOL_1024, ptr);
            }
            POOL_SIZE_2048 => {
                self.deallocate(POOL_2048, ptr);
            }
            POOL_SIZE_4096 => {
                self.deallocate(POOL_4096, ptr);
            }
            _ => {
                let size = layout.size();
                if size > self.pool.index(POOL_PGSZ).len() {
                    error!("the alloc size is to big");
                }
                self.pool.index_mut(POOL_PGSZ).push(ptr as usize);
            }
        };
    }
}
