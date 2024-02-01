use crate::{
    align_up,
    buddy::def::PAGE_SIZE,
    is_align,
    linklist::{def::*, link::Linkedlist},
    BuddyAllocator,
};
use core::{
    alloc::Layout,
    ops::{Index, IndexMut}, ptr::null_mut,
};
use xxos_log::{error, info};

/// 小内存分配器
/// 基于页内存分配器，使用了8 个内存池，分配对应大小的内存
/// 最大不超过一个页，超过一个页则调用页内存分配器直接获取
/// 对应大小内存
pub struct SlabAllocator {
    pub(crate) pool: [Linkedlist; POOL_COUNT],
    pub(crate) buddy: BuddyAllocator,
}

unsafe impl Send for SlabAllocator {}

impl SlabAllocator {
    pub const fn new() -> Self {
        Self {
            pool: [Linkedlist::new(); POOL_COUNT],
            buddy: BuddyAllocator::new(),
        }
    }

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
        self.buddy.init(bottom, top);
    }

    unsafe fn allocate<T>(&mut self, index: usize, layout: Layout) -> Option<*mut T> {
        info!("allocate the index is {}", index);

        if let Some(ptr) = self.pool.index_mut(index).pop_algin::<T>(layout.size()) {
            info!("it alloced!");
            Some(ptr)
        } else {
            info!("none value in pool , go to buddy to alloc new page!");
            info!("the size is {:#x}!", layout.size());
            let alloc_from_body = Layout::from_size_align(PGSZ, PGSZ).expect("err");
            //TODO it should have error handle
            let page = self.buddy.allocate(alloc_from_body).expect("None Page");

            let start = page;

            info!("it got a page {:#x}!", page);

            let end = start + PGSZ;
            info!("the end of the page is  {:#x}!", end);
            assert_eq!(self.pool.index(index).len(),0);

            self.pool.index_mut(index).init(start, end, layout.size());
            
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
        let layout = Self::align_layout(layout)?;
        info!("get regular size ,{}",layout.size());
        if layout.size() == 0 {return Ok(null_mut());}
        let mut ptr = None;
        let size_arr = [32,64,128,256,512,1024,2048,4096];
        let mut index = 0;
        for pool_size in size_arr {
            if layout.size() == pool_size {
                info!("allocer in index {} the size is {}",index,layout.size());
                ptr = self.allocate(index, layout);
                break;
            }else {
                index += 1;
            }
        };

        if ptr.is_none(){
            info!("the request size is more the pgsz , {}",layout.size());
            ptr = self.buddy.allocate(layout).ok().map(|x| x as *mut _);
        };
        //Todo it should have error handing
        ptr.ok_or(())
    }

    unsafe fn deallocate(&mut self, index: usize, ptr: *mut u8) {
        self.pool.index_mut(index).push(ptr as usize)
    }

    pub fn is_aready_algin_for_pool(&self,layout: Layout) -> bool{
        let size_arr = [32,64,128,256,512,1024,2048,4096];
        let mut res = false;
        for size in size_arr {
            if layout.size() == size {
                res = true;
                break;
            }
        }
        if layout.size() > PGSZ && is_align!(layout.size(),PGSZ) {
            res = true
        }
        res
    }

    pub unsafe fn deallocate_fit(&mut self, ptr: *mut u8, layout: Layout) {
        if self.is_aready_algin_for_pool(layout) {
            error!("the free size {} isn't a regular size ",layout.size());
            panic!("err free")
        }
        let size_arr = [32,64,128,256,512,1024,2048,4096];
        let mut index = 0;
        for size in size_arr {
            if layout.size() == size{
                self.deallocate(index, ptr);
            } else {
                index += 1;
            }
        }

        if layout.size() > PGSZ{
            //Todo it should have error handing
            self.buddy.deallocate(ptr as usize, layout.size()).expect("error the buddy free error");
            return;
        }
        error!("in slab dealloc_fid not should run here")
    }
}
