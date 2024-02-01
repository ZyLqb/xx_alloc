#![no_std]
#![feature(const_mut_refs)]
#![feature(const_for)]
#![feature(const_trait_impl)]
#![feature(exclusive_range_pattern)]
mod bintree;
mod buddy;
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
    use xxos_log::{info, init_log, warn, WriteLog};
    extern crate std;
    use crate::slab::slab_lock::LockedSlab;
    struct PT;
    impl WriteLog for PT {
        fn print(&self, log_content: core::fmt::Arguments) {
            println!("{}", log_content)
        }
    }
    #[test]
    fn test() {
        init_log(&PT, xxos_log::Level::INFO);
        let heap_arr = [0usize; 4096 * 17];
        let bottom = &heap_arr[0] as *const _ as usize;
        let top = &heap_arr[4096 * 17 - 1] as *const _ as usize;
        unsafe {
            let heap = LockedSlab::new_uninit();
            heap.init(bottom, top);
            for _ in 0..10 {
                let layout = Layout::from_size_align(290, 8).unwrap();
                let a = heap.alloc(layout);

                warn!("a {:#x}", a as usize);
                heap.dealloc(a, layout)
            }
        }
    }
}
