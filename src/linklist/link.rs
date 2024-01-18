use crate::align_down;
use crate::align_up;

use super::node::Node;
use core::ptr::null_mut;
use xxos_log::info;
use xxos_log::LOG;

#[derive(Clone, Copy)]
pub(crate) struct Linkedlist {
    head: *mut Node,
}

pub struct LinkedlistIter {
    current: *mut Node,
}

impl Iterator for LinkedlistIter {
    type Item = *mut Node;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let head = self.current;

            if (*head).next.is_null() {
                None
            } else {
                let item = self.current;
                let next = (*item).next;
                self.current = next;
                Some(item)
            }
        }
    }
}

impl Linkedlist {
    #[inline]
    pub const fn new() -> Self {
        Self { head: null_mut() }
    }

    pub fn iter(&self) -> LinkedlistIter {
        LinkedlistIter { current: self.head }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub fn len(&self) -> usize {
        let mut len = 0;

        for _ in self.iter() {
            len += 1;
        }

        len
    }

    pub unsafe fn init(&mut self, start: usize, end: usize, chunk_size: usize) {
        info!("start before align: {:#x}", start);
        info!("end before align: {:#x}", end);

        let start = align_up!(start, chunk_size);
        let end = align_down!(end, chunk_size);

        info!("chunk size: {:#x}", chunk_size);
        info!("start after align: {:#x}", start);
        info!("end after align: {:#x}", end);

        for address in (start..end).step_by(chunk_size) {
            let head = Node::to_mut_node_ptr(address);
            if address + chunk_size == end {
                unsafe { (*head).next = null_mut() }
            }
            let next = Node::to_mut_node_ptr(address + chunk_size);
            unsafe {
                (*head).next = next;
            }
        }

        self.head = Node::to_mut_node_ptr(start);

        info!("init ok the len is {}", self.len())
    }

    pub unsafe fn pop<T>(&mut self) -> Option<*mut T> {
        if !self.head.is_null() {
            let head = self.head;
            self.head = (*head).next;
            Some(head as *mut T)
        } else {
            None
        }
    }

    pub unsafe fn push(&mut self, address: usize) {
        let head = Node::to_mut_node_ptr(address);
        assert!(!head.is_null());

        (*head).next = self.head;

        self.head = head;
    }

    pub unsafe fn push_tail(&mut self, address: usize) {
        let tail = Node::to_mut_node_ptr(address);
        assert!(!tail.is_null());

        let mut current = self.head;
        while !(*(self.head)).next.is_null() {
            current = (*current).next
        }

        (*current).next = tail;
        (*tail).next = null_mut();
    }
}

#[cfg(test)]
mod tests {
    use xxos_log::{info, init_log};

    use super::*;
    use crate::linklist::def::POOL_SIZE_64;
    extern crate std;

    #[test]
    fn it_works() {
        use std::println;
        use xxos_log::WriteLog;

        struct PT;

        impl WriteLog for PT {
            fn print(&self, log_content: core::fmt::Arguments) {
                println!("{}", log_content)
            }
        }

        let heap = [0u8; 4096];

        unsafe {
            init_log(&PT, xxos_log::Level::INFO);

            let start = &heap as *const _ as usize;
            let end = &heap[4095] as *const _ as usize;
            let mut linked_list = Linkedlist::new();

            std::println!(
                "test size: {:#x}\ntest len: {}",
                (end - start),
                (end - start) / 64
            );

            linked_list.init(start, end, POOL_SIZE_64);

            assert_eq!((end - start) / 64, linked_list.len());

            for (i, items) in linked_list.iter().enumerate() {
                assert_eq!(
                    align_up!(start, POOL_SIZE_64) + POOL_SIZE_64 * i,
                    items as usize
                );
                //println!("{}: {:#x}", i, items as usize);
            }
        }
    }
}
