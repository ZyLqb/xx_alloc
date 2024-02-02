#![no_std]
#![feature(const_mut_refs)]
#![feature(const_for)]
#![feature(const_trait_impl)]
#![feature(exclusive_range_pattern)]
mod bintree;
mod buddy;
mod def;
mod linklist;
mod macros;
mod slab;

//pub use bintree::treemap::TreeMap;
pub use buddy::buddy_allocator::BuddyAllocator;
pub use slab::slab_lock::LockedSlab;

#[cfg(test)]
mod tests {
    use core::alloc::{GlobalAlloc, Layout};
    use std::println;
    use xxos_log::WriteLog;
    extern crate std;
    use crate::{def::PGSZ, slab::slab_lock::LockedSlab};
    struct PT;
    impl WriteLog for PT {
        fn print(&self, log_content: core::fmt::Arguments) {
            println!("{}", log_content)
        }
    }
    #[test]
    fn test_alloc_small() {
        let heap_arr = [0usize; 4096 * 10];
        let bottom = &heap_arr[0] as *const _ as usize;
        let top = &heap_arr[4096 * 10 - 1] as *const _ as usize;
        unsafe {
            let heap = LockedSlab::new_uninit();
            heap.init(bottom, top);
            let mut now = 0;
            for i in 0..10 {
                let last = now;
                let layout = Layout::from_size_align(290, 8).unwrap();
                now = heap.alloc(layout) as usize;
                if i != 0 {
                    assert_eq!(last + 512, now);
                }
            }
        }
    }

    #[test]
    fn test_free_small() {
        let heap_arr = [0usize; 4096 * 10];
        let bottom = &heap_arr[0] as *const _ as usize;
        let top = &heap_arr[4096 * 10 - 1] as *const _ as usize;
        unsafe {
            let heap = LockedSlab::new_uninit();
            heap.init(bottom, top);
            let mut now = 0;
            for i in 0..10 {
                let last = now;
                let layout = Layout::from_size_align(290, 8).unwrap();
                now = heap.alloc(layout) as usize;
                heap.dealloc(now as *mut _, layout);
                if i != 0 {
                    assert_eq!(last, now);
                }
            }
        }
    }

    #[test]
    fn test_alloc_big() {
        let heap_arr = [0usize; 4096 * 10];
        let bottom = &heap_arr[0] as *const _ as usize;
        let top = &heap_arr[4096 * 10 - 1] as *const _ as usize;
        unsafe {
            let heap = LockedSlab::new_uninit();
            heap.init(bottom, top);
            let mut now = 0;
            for i in 0..3 {
                let last = now;
                let layout = Layout::from_size_align(PGSZ * 2, 8).unwrap();
                now = heap.alloc(layout) as usize;
                if i != 0 {
                    assert_eq!(last + PGSZ * 2, now);
                }
            }
        }
    }

    #[test]
    fn test_free_big() {
        let heap_arr = [0usize; 4096 * 10];
        let bottom = &heap_arr[0] as *const _ as usize;
        let top = &heap_arr[4096 * 10 - 1] as *const _ as usize;
        unsafe {
            let heap = LockedSlab::new_uninit();
            heap.init(bottom, top);
            let mut now = 0;
            for i in 0..3 {
                let last = now;
                let layout = Layout::from_size_align(PGSZ * 2, 8).unwrap();
                now = heap.alloc(layout) as usize;
                heap.dealloc(now as *mut _, layout);
                if i != 0 {
                    assert_eq!(last, now);
                }
            }
        }
    }
}
