use super::def::{MemPtr, MAX_PAGES, PAGE_SIZE};
use crate::{
    align_down, align_up,
    bintree::tree::{BinTree, TreeErr},
    is_align,
};
use core::{alloc::Layout, mem::size_of, ptr::null_mut};

#[derive(Debug)]
pub enum BuddyErr {
    None,
    NotEnough,
    NotFound,
    WrongSize,
    WrongAddr,
}

impl From<TreeErr> for BuddyErr {
    fn from(value: TreeErr) -> Self {
        match value {
            TreeErr::WrongSize => Self::WrongSize,
            TreeErr::NotFound => Self::NotFound,
            TreeErr::NotEnough => Self::NotEnough,
        }
    }
}

/// 页内存分配器
/// 用来分配连续的页内存，使用完全二叉树来管理
/// 因此管理的页数为2的幂
/// Example:
/// ```
/// let mut buddy = BuddyAllocator::new(bottom, top);
/// let mut addr1 = buddy.allocate(Layout::from_size_align(PAGE_SIZE, PAGE_SIZE).unwrap());
/// let mut addr2 = buddy.allocate(Layout::from_size_align(PAGE_SIZE << 1, PAGE_SIZE).unwrap());
/// let _ = buddy.deallocate(addr1.unwrap(), PAGE_SIZE);
/// ```
#[derive(Debug)]
pub struct BuddyAllocator {
    zone: *mut BinTree, // 二叉树
    page_counts: usize, // 剩余空闲页
}

#[allow(unused)]
impl BuddyAllocator {
    pub const fn new() -> Self {
        Self {
            zone: null_mut(),
            page_counts: 0,
        }
    }

    // 初始化zone
    // 需要起始地址和总内存大小
    /// # Safety
    pub unsafe fn init(&mut self, bottom: MemPtr, top: MemPtr) {
        let start = align_up!(bottom, PAGE_SIZE);
        let end = align_down!(top, PAGE_SIZE);
        let mut zone = start as *mut BinTree;
        let mut page_counts = (end - start) / PAGE_SIZE + 1;

        if page_counts > MAX_PAGES {
            panic!("size too big.");
        }

        self.zone = start as *mut BinTree;
        self.page_counts = page_counts;

        match (*self.zone).init(self.zone as usize, PAGE_SIZE * self.page_counts) {
            Ok(counts) => {
                // 由于直接使用待管理内存的前几页保存该分配器
                // 因此设置前三页为used
                let used = align_up!(size_of::<BinTree>(), PAGE_SIZE) / PAGE_SIZE;
                let index = (*self.zone).get_index((*self.zone).level);

                for i in 0..used {
                    (*self.zone).use_page(index + i);
                }

                self.page_counts = counts - used;
            }
            Err(_) => {
                panic!("");
            }
        }
    }
    ///
    /// # Safety
    ///
    pub unsafe fn init_new(&mut self, bottom: MemPtr, top: MemPtr) {
        let start: usize = align_up!(bottom, PAGE_SIZE);
        let end = align_down!(top, PAGE_SIZE);
        let mut zone = start as *mut BinTree;
        let mut page_counts = (end - start) / PAGE_SIZE + 1;

        self.zone = zone;
        self.page_counts = page_counts;

        match (*self.zone).init(self.zone as usize, PAGE_SIZE * self.page_counts) {
            Ok(counts) => {
                // 由于直接使用待管理内存的前几页保存该分配器
                // 因此设置前三页为used
                let used = align_up!(size_of::<BinTree>(), PAGE_SIZE) / PAGE_SIZE;
                let index = (*self.zone).get_index((*self.zone).level);

                for i in 0..used {
                    (*self.zone).use_page(index + i);
                }

                self.page_counts = counts - used;
            }
            Err(err) => panic!("{:?}", err),
        }
    }

    // 分配内存，需要提供待分配内存大小
    /// # Safety
    pub unsafe fn allocate(&mut self, layout: Layout) -> Result<MemPtr, BuddyErr> {
        let size = layout.size();
        let align_size = layout.align();
        let mem_size = align_up!(size, PAGE_SIZE);

        if self.page_counts == 0 {
            Err(BuddyErr::None)
        } else {
            let mut addr = 0;
            let counts = size / PAGE_SIZE;

            if counts > self.page_counts {
                return Err(BuddyErr::NotEnough);
            }

            // 剩余页面足够时，找到对应的unused节点并设置为used
            // 剩余页面减少
            let mut idx = (*self.zone).find(mem_size, false)?;
            let max_idx = (*self.zone).get_index((*self.zone).get_level(size));

            // 找到与layout对齐的地址
            addr = (*self.zone).get_value(idx);
            while idx < max_idx && !is_align!(addr, align_size) {
                idx += 1;
                addr = (*self.zone).get_value(idx);
            }

            if idx != max_idx {
                // 找到子树的最左节点
                let mut left_leaf = idx;
                let max_leaf = (*self.zone).max_node();
                while (*self.zone).find_left_child(left_leaf) <= max_leaf {
                    left_leaf = (*self.zone).find_left_child(left_leaf);
                }

                // 检查连续的页是否可用
                if (*self.zone).can_use(left_leaf, counts) {
                    (*self.zone).use_mem(idx);
                    self.page_counts -= counts;
                    Ok(addr)
                } else {
                    Err(BuddyErr::NotFound)
                }
            } else {
                Err(BuddyErr::NotFound)
            }
        }
    }

    // 释放内存，需要提供起始地址和内存大小
    /// # Safety
    pub unsafe fn deallocate(&mut self, addr: MemPtr, size: usize) -> Result<usize, BuddyErr> {
        let counts = size / PAGE_SIZE;

        // 地址和大小需要对齐
        if is_align!(addr, PAGE_SIZE) {
            if is_align!(size, PAGE_SIZE) {
                let mut idx = 0;

                // 找到对应节点并设置其为unused
                let index = (*self.zone).find_match(size, addr, true)?;
                (*self.zone).unuse_mem(index);
                idx = index;

                Ok(idx)
            } else {
                Err(BuddyErr::WrongAddr)
            }
        } else {
            Err(BuddyErr::WrongSize)
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
pub mod buddy_tests {
    extern crate std;
    use super::BuddyAllocator;
    use crate::align_up;
    use crate::bintree::def::MIN_SIZE;
    use crate::buddy::def::PAGE_SIZE;
    use core::alloc::Layout;
    use std::{panic, println};
    use xxos_log::{info, init_log, WriteLog};
    struct PT;

    impl WriteLog for PT {
        fn print(&self, log_content: core::fmt::Arguments) {
            println!("{}", log_content);
        }
    }

    #[test]
    fn buddy_test() {
        init_log(&PT, xxos_log::Level::INFO);

        const PAGE_COUNTS: usize = 16;
        let test_mem: [usize; PAGE_SIZE * PAGE_COUNTS / 8] = [0; PAGE_SIZE * PAGE_COUNTS / 8];
        info!("test_mem_size: {:x} Bytes", PAGE_COUNTS * PAGE_SIZE);

        let bottom = test_mem.as_ptr() as usize;
        let top = bottom + PAGE_COUNTS * PAGE_SIZE;

        info!(
            "BuddyAllocator::new(bottom: {:#x}, top: {:#x})",
            bottom, top
        );

        let mut buddy = BuddyAllocator::new();
        unsafe { buddy.init(bottom, top) };

        assert_eq!(align_up!(bottom, MIN_SIZE), buddy.zone as usize);

        info!(
            "BuddyAllocator::allocate(size: {:x}, align_size: {:#x})",
            PAGE_SIZE,
            PAGE_SIZE << 1
        );
        let mut addr1 =
            unsafe { buddy.allocate(Layout::from_size_align(PAGE_SIZE, PAGE_SIZE << 1).unwrap()) };
        match addr1 {
            Ok(addr) => {
                info!("allocate addr1: {:#x}", addr);
                assert_eq!(
                    align_up!(bottom + 3 * PAGE_SIZE, PAGE_SIZE),
                    addr,
                    "\nThis result has related to the root address, example my root address is {:#x} so my first allocate aligned {:#x} address is page no.4(root + 3 * PGSZ = {:#x})",
                    buddy.zone as usize,PAGE_SIZE<<1,align_up!(bottom+3*PAGE_SIZE,PAGE_SIZE)
                );
            }
            Err(_) => {
                panic!("")
            }
        }

        info!(
            "BuddyAllocator::allocate({:#x}, align_size: {:#x})",
            PAGE_SIZE << 1,
            PAGE_SIZE
        );
        let mut addr2 =
            unsafe { buddy.allocate(Layout::from_size_align(PAGE_SIZE << 1, PAGE_SIZE).unwrap()) };
        match addr2 {
            Ok(addr) => {
                info!("allocate addr2: {:#x}", addr);
                assert_eq!(align_up!(bottom + 4 * PAGE_SIZE, PAGE_SIZE), addr);
            }
            Err(_) => {
                panic!("");
            }
        }

        info!(
            "BuddyAllocator::allocate({:#x}, align_size: {:#x})",
            PAGE_SIZE, PAGE_SIZE
        );
        let addr3 =
            unsafe { buddy.allocate(Layout::from_size_align(PAGE_SIZE, PAGE_SIZE).unwrap()) };
        match addr3 {
            Ok(addr) => {
                info!("allocate addr3: {:#x}", addr);
                assert_eq!(align_up!(bottom + 6 * PAGE_SIZE, PAGE_SIZE), addr,);
            }
            Err(_) => {
                panic!("");
            }
        }

        info!("BuddyAllocator::deallocate({:#x})", addr1.as_ref().unwrap());
        let free1 = unsafe { buddy.deallocate(addr1.unwrap(), PAGE_SIZE) }.unwrap();
        assert_eq!(18, free1);
        info!("BuddyAllocator::deallocate({:#x})", addr2.as_ref().unwrap());
        let free2 = unsafe { buddy.deallocate(addr2.unwrap(), PAGE_SIZE << 1) }.unwrap();
        assert_eq!(9, free2);

        addr1 = unsafe { buddy.allocate(Layout::from_size_align(PAGE_SIZE, PAGE_SIZE).unwrap()) };
        match addr1 {
            Ok(addr) => {
                info!("allocate addr: {:#x}", addr);
                assert_eq!(align_up!(bottom + 3 * PAGE_SIZE, PAGE_SIZE), addr);
            }
            Err(_) => {
                panic!("");
            }
        }

        addr2 =
            unsafe { buddy.allocate(Layout::from_size_align(PAGE_SIZE << 1, PAGE_SIZE).unwrap()) };
        match addr2 {
            Ok(addr) => {
                info!("allocate addr: {:#x}", addr);
                assert_eq!(align_up!(bottom + 4 * PAGE_SIZE, PAGE_SIZE), addr);
            }
            Err(_) => {
                panic!("");
            }
        }
    }
}
