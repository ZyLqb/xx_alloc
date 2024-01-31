use crate::{
    align_up,
    buddy::def::PAGE_SIZE,
    is_align,
    linklist::{def::*, link::Linkedlist},
    BuddyAllocator,
};
use core::{
    alloc::Layout,
    ops::{Index, IndexMut},
};
use xxos_log::{error, info, warn};

/// 小内存分配器
/// 基于页内存分配器，使用了8 个内存池，分配对应大小的内存
/// 最大不超过一个页，超过一个页则调用页内存分配器直接获取
/// 对应大小内存
pub struct SlabAllocator {
    pub(crate) pool: [Linkedlist; POOL_COUNT],
    pub(crate) buddy: BuddyAllocator
}

unsafe impl Send for SlabAllocator {}

impl SlabAllocator {
    pub const fn new() -> Self {
        Self {
            pool: [Linkedlist::new(); POOL_COUNT],
            buddy: BuddyAllocator::new_new(),
        }
    }

    // pub fn is_pool_empty(&self, index: usize) -> bool {
    //     self.pool[index].is_empty()
    // }

    pub fn align_layout(layout: Layout) -> Result<Layout, ()> {
        fn find_fit_size(size: usize) -> usize {
            match size {
                0 => 0,
                sz if sz > 0 && sz <= POOL_SIZE_32 => POOL_SIZE_32,
                sz if sz > POOL_SIZE_32 && sz <= POOL_SIZE_64 => POOL_SIZE_64,
                sz if sz > POOL_SIZE_64 && sz <= POOL_SIZE_128 => POOL_SIZE_128,
                sz if sz > POOL_SIZE_128 && sz <= POOL_SIZE_256 => POOL_SIZE_256,
                sz if sz > POOL_SIZE_256 && sz <= POOL_SIZE_512 => POOL_SIZE_512,
                sz if sz > POOL_SIZE_512 && sz <= POOL_SIZE_1024 => POOL_SIZE_1024,
                sz if sz > POOL_SIZE_1024 && sz <= POOL_SIZE_2048 => POOL_SIZE_2048,
                sz if sz > POOL_SIZE_2048 && sz <= POOL_SIZE_4096 => POOL_SIZE_4096,
                _ => align_up!(size, PGSZ),
            }
        }

        fn get_algin(v1: usize, v2: usize) -> usize {
            core::cmp::max(v1, v2)
        }

        info!(
            "the layout before find  is size {} algin {}",
            layout.size(),
            layout.align()
        );

        let fit_size = find_fit_size(layout.size());
        let algin = get_algin(layout.align(), fit_size);

        info!("the layout is size {:#x} algin {:#x}", fit_size, algin);
        Layout::from_size_align(fit_size, algin).map_err(|_| ())
    }

    pub unsafe fn init(&mut self, bottom: usize, top: usize) {
        self.pool.index_mut(POOL_PGSZ).init(bottom, top, PGSZ);
    }

    unsafe fn allocate<T>(&mut self, index: usize, layout: Layout) -> Option<*mut T> {
        info!("allocate the index is {}", index);

        if let Some(ptr) = self.pool.index_mut(index).pop_algin::<T>(layout.size()) {
            info!("it alloced!");
            Some(ptr)
        } else {
            //tode it should alloc in buddy
            info!("none value in pool , it go find new page!");
            info!("the size is {:#x}!", layout.size());

            // let page = self
            //     .pool
            //     .index_mut(POOL_PGSZ)
            //     .pop::<T>()
            //     .expect("None Page");
            
            //TODO it should have error handle
            let page = self.buddy.allocate(layout.size()).expect("None Page");
            
            let start = page;
            info!("it got a page {:#x}!", page);

            let end = start + PGSZ;

            info!("free the page {:#x} in page pool {}", start, index);
            for address in (start..end).step_by(layout.size()) {
                self.pool.index_mut(index).push(address)
            }
            if layout.align() > PAGE_SIZE {
                error!("the algin is too big , plese give a samller algin");
                return None;
            }
            let ptr = self
                .pool
                .index_mut(index)
                .pop::<T>()
                .expect("it no mem in this pool");
            Some(ptr)
        }
    }

    pub unsafe fn allocate_fit(&mut self, layout: Layout) -> Result<*mut u8, ()> {
        info!("allocate start ");

        let fit_layout = Self::align_layout(layout)?;
        let mut ptr = None;

        match fit_layout.size() {
            POOL_SIZE_32 => {
                info!("allocer in 32");
                ptr = self.allocate(POOL_32, fit_layout);
            }
            POOL_SIZE_64 => {
                info!("allocer in 64");
                ptr = self.allocate(POOL_64, fit_layout);
            }
            POOL_SIZE_128 => {
                info!("allocer in 128");
                ptr = self.allocate(POOL_128, fit_layout);
            }
            POOL_SIZE_256 => {
                info!("allocer in 256");
                ptr = self.allocate(POOL_256, fit_layout);
            }
            POOL_SIZE_512 => {
                info!("allocer in 512");
                ptr = self.allocate(POOL_512, fit_layout);
            }
            POOL_SIZE_1024 => {
                info!("allocer in 1024");
                ptr = self.allocate(POOL_1024, fit_layout);
            }
            POOL_SIZE_2048 => {
                info!("allocer in 2048");
                ptr = self.allocate(POOL_2048, fit_layout);
            }
            POOL_SIZE_4096 => {
                info!("allocer in 4096");
                ptr = self.allocate(POOL_4096, fit_layout);
            }
            _ => {
                //TODO : alloc the page in buddy
                info!("allocer start other");
                let size = fit_layout.size();
                let align_size = layout.align();

                if size > PGSZ {
                    error!("the alloc size is too big");
                    return Err(());
                }

                loop {
                    let ptr = self
                        .pool
                        .index_mut(POOL_PGSZ)
                        .pop::<u8>()
                        .expect("None value");

                    if is_align!(ptr as usize, align_size) {
                        break;
                    } else {
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
