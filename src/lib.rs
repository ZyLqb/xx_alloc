#![no_std]
#![feature(const_mut_refs)]
#![feature(const_for)]
#![feature(const_trait_impl)]
#![feature(exclusive_range_pattern)]
mod bintree;
mod linklist;
mod slab;

pub use bintree::treemap::TreeMap;
pub use slab::slab_lock::LockedSlab;

//#[cfg(test)]
//mod tests {
//    use core::alloc::Layout;
//    use std::println;
//    use xxos_log::{info, init_log, WriteLog};
//    extern crate std;
//    use crate::heap::Heap;
//    struct PT;
//    impl WriteLog for PT {
//        fn print(&self, log_content: core::fmt::Arguments) {
//            println!("{}", log_content)
//        }
//    }
//    #[test]
//    fn test() {
//        use xxos_log::LOG;
//        init_log(&PT, xxos_log::Level::INFO);
//        let heap_arr = [0usize; 4096 * 17];
//        let bottom = &heap_arr[0] as *const _ as usize;
//        let top = &heap_arr[4096 * 17 - 1] as *const _ as usize;
//        unsafe {
//            let mut heap = Heap::new_uninit();
//            heap.init(bottom, top);
//            let layout = Layout::from_size_align(290, 8).unwrap();
//            let a = heap.allocate_fit(layout).unwrap();
//            info!("a {}", a as usize)
//        }
//    }
//}
