use core::ptr::{null_mut, NonNull};

use super::{align_to_down, align_to_up};
use super::{def::PGSZ, node::Node};
pub(crate) struct Linkedlist {
    head: *mut Node,
    top: usize,
    bottom: usize,
}

impl Iterator for Iter {
    type Item = *mut Node;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let head = self.current;
            if (*head).prev.is_null() {
                None
            }else {
                let item = self.current;
                let next = (*item).prev;
                self.current = next;
                Some(item)    
            }
        }
    }
}

pub struct Iter {
    current: *mut Node,
}


impl Linkedlist {
    #[inline]
    pub const fn new() -> Self {
        Self {
            head: null_mut(),
            top: 0,
            bottom: 0,
        }
    }

    pub fn iter(&self) -> Iter{
        Iter { current: self.head }
    }
    pub fn init(&mut self,start: usize, size: usize) {
        let end = start + size;
        let start = align_to_up(start, PGSZ);
        let end = align_to_down(end, PGSZ);
        self.bottom = start;
        self.top = end;
        for address in (start..end).step_by(PGSZ) {
            let head = Node::to_current(address);
            if address+PGSZ == end {
                unsafe {
                    (*head).prev = null_mut()
                }
            }
            
            let prev = Node::to_prev(address + PGSZ);
            unsafe {
                (*head).prev = prev;
            }
        }
        self.head = Node::to_current(start)
    }

    pub unsafe fn alloc<T>(&mut self) -> *mut T{
        let head = self.head;
        self.head = (*head).prev;
        head  as *mut T
    }

    pub unsafe fn dealloc(&mut self, address: usize){
        let head = Node::to_current(address);
        (*head).prev = self.head;
        self.head = head; 
    }
}


#[cfg(test)]
mod tests {
    use crate::linklist::{self, node};
    use super::*;
    extern crate std;
    
    #[test]
    fn it_works() {
        let heap = [0u8;4096*2];
        unsafe {
            let start = &heap[0] as *const _ as usize;
            let end = &heap[8191] as *const _ as usize;
            std::println!("{}",(end - start));
            let mut linked_list = Linkedlist::new();
            linked_list.init(start, end - start);
            let node:*mut u8 = linked_list.alloc();
            use core::intrinsics::volatile_set_memory;
            volatile_set_memory(node, 65 , PGSZ);
            let back_to_array: &[u8; PGSZ/2] = core::mem::transmute(node);
            let bac = back_to_array.as_slice(); 
            let s = std::str::from_utf8(bac).unwrap();
            std::println!("{:?}",s);
            std::println!("the head is{}",node as usize);

            linked_list.dealloc(node as usize);
            let next = (*(linked_list.head)).prev as usize;
            std::println!("the prev is{}",next );
            for (i,_) in linked_list.iter().enumerate() {
                std::println!("the len is {}",i)
            }
        }
    }
}
