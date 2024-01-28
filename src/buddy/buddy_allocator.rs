use super::def::PAGE_SIZE;

use crate::{
    align_down, align_up,
    bintree::{
        def::{MAX_SIZE, MIN_SIZE},
        tree::BinTree,
    },
    is_align,
};
use core::mem::size_of;

type MemPtr = usize;

/// 页内存分配器
/// 用来分配连续的页内存，使用完全二叉树来管理
/// 因此管理的页数为2的幂
#[derive(Debug)]
pub struct BuddyAllocator {
    zone: *mut BinTree,
    page_counts: usize,
}

#[allow(unused)]
impl BuddyAllocator {
    fn new(bottom: MemPtr, top: MemPtr) -> Self {
        let start = align_up!(bottom, PAGE_SIZE);
        let end = align_down!(top, PAGE_SIZE);
        let mut zone = start as *mut BinTree;
        let mut page_counts = (end - start) / PAGE_SIZE + 1;

        if page_counts > MAX_SIZE {
            panic!("size too big.");
        }

        unsafe {
            match (*zone).init(start, PAGE_SIZE * page_counts) {
                Ok(counts) => {
                    let used = align_up!(size_of::<BinTree>(), PAGE_SIZE) / PAGE_SIZE;
                    let index = (*zone).get_index((*zone).level);

                    for i in 0..used {
                        (*zone).use_page(index + i);
                    }

                    page_counts = counts - used;
                }
                Err(err) => panic!("{}", err),
            }
        }

        Self { zone, page_counts }
    }

    pub fn allocate(&mut self, size: usize) -> Result<MemPtr, &'static str> {
        let mem_size = align_up!(size, PAGE_SIZE);

        if self.page_counts == 0 {
            Err("page is not enough")
        } else {
            let mut addr = 0;
            let counts = size / MIN_SIZE;

            if counts > self.page_counts {
                return Err("buddy::allocate: have not enough page");
            }

            unsafe {
                if let Ok(idx) = (*self.zone).find_unused(mem_size) {
                    addr = (*self.zone).get_value(idx);
                    (*self.zone).use_mem(idx);
                    self.page_counts -= counts;
                } else {
                    return Err("buddy::allocate");
                }
            }

            Ok(addr)
        }
    }

    pub fn deallocate(&mut self, addr: MemPtr, size: usize) -> Result<usize, &'static str> {
        let counts = size / MIN_SIZE;
        if is_align!(addr, MIN_SIZE) && is_align!(size, MIN_SIZE) {
            unsafe {
                let mut idx = (*self.zone).find_used(size).unwrap();
                let max_node = (*self.zone).max_node();
                while idx < max_node {
                    if addr == (*self.zone).get_value(idx) {
                        (*self.zone).unuse_mem(idx);
                        break;
                    }

                    idx += 1;
                }
            }

            self.page_counts += counts;
            Ok(self.page_counts)
        } else {
            Err("addr or size is wrong")
        }
    }
}

#[cfg(test)]
pub mod buddy_tests {
    extern crate std;
    use super::BuddyAllocator;
    use crate::align_up;
    use crate::bintree::def::MIN_SIZE;
    use crate::buddy::def::PAGE_SIZE;
    use std::{panic, println};
    use xxos_log::LOG;
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
        info!("test_mem_size: 0x{:x} Bytes", PAGE_COUNTS * PAGE_SIZE);

        let bottom = test_mem.as_ptr() as usize;
        let top = bottom + PAGE_COUNTS * PAGE_SIZE;

        info!("BuddyAllocator::new(bottom, top)");
        info!("top: {:#x} bottom: {:#x}", top, bottom);

        let mut buddy = BuddyAllocator::new(bottom, top);

        info!("BuddyAllocator::new(bottom, top) end");

        assert_eq!(align_up!(bottom, MIN_SIZE), buddy.zone as usize);

        info!("BuddyAllocator::allocate(0x{:x})", PAGE_SIZE);
        let mut addr1 = buddy.allocate(PAGE_SIZE);
        match addr1 {
            Ok(addr) => {
                info!("allocate addr: {:#x}", addr);
                assert_eq!(align_up!(bottom + 3 * PAGE_SIZE, PAGE_SIZE), addr);
            }
            Err(err) => {
                panic!("{}", err)
            }
        }

        info!("BuddyAllocator::allocate(0x{:x})", PAGE_SIZE << 1);
        let mut addr2 = buddy.allocate(PAGE_SIZE << 1);
        match addr2 {
            Ok(addr) => {
                info!("allocate addr: {:#x}", addr);
                assert_eq!(align_up!(bottom + 4 * PAGE_SIZE, PAGE_SIZE), addr);
            }
            Err(err) => {
                panic!("{}", err)
            }
        }

        info!("BuddyAllocator::allocate(0x{:x})", PAGE_SIZE);
        let addr3 = buddy.allocate(PAGE_SIZE);
        match addr3 {
            Ok(addr) => {
                info!("allocate addr: {:#x}", addr);
                assert_eq!(align_up!(bottom + 6 * PAGE_SIZE, PAGE_SIZE), addr);
            }
            Err(err) => {
                panic!("{}", err)
            }
        }

        info!("BuddyAllocator::deallocate(0x{:x})", addr1.unwrap());
        let free1 = buddy.deallocate(addr1.unwrap(), PAGE_SIZE);
        assert_eq!(10, free1.unwrap());
        info!("BuddyAllocator::deallocate(0x{:x})", addr2.unwrap());
        let free2 = buddy.deallocate(addr2.unwrap(), PAGE_SIZE * 2);
        assert_eq!(12, free2.unwrap());

        addr1 = buddy.allocate(PAGE_SIZE);
        match addr1 {
            Ok(addr) => {
                info!("allocate addr: {:#x}", addr);
                assert_eq!(align_up!(bottom + 3 * PAGE_SIZE, PAGE_SIZE), addr);
            }
            Err(err) => {
                panic!("{}", err)
            }
        }

        addr2 = buddy.allocate(PAGE_SIZE << 1);
        match addr2 {
            Ok(addr) => {
                info!("allocate addr: {:#x}", addr);
                assert_eq!(align_up!(bottom + 4 * PAGE_SIZE, PAGE_SIZE), addr);
            }
            Err(err) => {
                panic!("{}", err)
            }
        }

        /* unsafe {
            info!("{:x?}", (*buddy.zone).bitmap);
        } */
    }
}
