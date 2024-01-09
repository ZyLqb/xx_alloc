#![no_std]
#![feature(core_intrinsics)]
#![feature(const_mut_refs)]
#![feature(const_for)]
#![feature(const_trait_impl)]
mod linklist;
mod heap;
use xx_mutex_lock::Mutex;
#[cfg(test)]
mod tests {
    //use super::*;
    //extern crate std;
    //#[test]
    // fn it_works() {
    //     let c = std::boxed::Box::new(1usize);
    //     let d = std::boxed::Box::new(2usize);
        
    //     unsafe {
    //         #[repr(transparent)]
    //         struct Node {
    //             pub next: *mut Node
    //         }
    //         let ptr = std::boxed::Box::leak(c) as *mut _ as usize;
    //         let ptr2 = std::boxed::Box::leak(d) as *mut _ as usize;
    //         std::println!("now: {:?} next: {:?}",ptr,ptr2);
    //         let node = ptr as *mut Node;
    //         let next_node = ptr2 as *mut Node;

    //         (*node).next = next_node;
            
    //         let num_next = (*node).next as usize as *mut usize;
    //         let num = node as usize as *mut usize;

    //         let now_ptr = num as usize;
    //         std::println!("now: {} next: {} now_ptr {}",*num,*num_next,now_ptr)
    //     }
    //     // std::println!("{}",c);
    //     // let result = add(2, 2);
    //     // assert_eq!(result, 4);
    // }


    use crate::heap::LockedHeap;
    extern crate alloc;
    //extern crate std;
    #[test]
    fn test(){
        //std::println!("{:?}","aaa");
        #[global_allocator]
        static ALLOCATOR: LockedHeap = LockedHeap::new();
        
        let arr = [0usize;4096*8];
        let bottom = &arr[0] as *const _ as usize;
        let top = &arr[4096*8-1] as *const _ as usize;

        ALLOCATOR.init(bottom, top);
        //std::println!("{:?}","bbb");
        use alloc::vec::Vec;
        let mut vec:Vec<usize> = Vec::with_capacity(10);
        vec.push(1);
        //std::println!("{:?}",vec)
    }
}
