use super::node::Node;
use super::{align_to_down, align_to_up};
use core::ptr::null_mut;
use xxos_log::info;
#[derive(Clone, Copy)]
pub(crate) struct Linkedlist {
    size: usize,
    head: *mut Node,
}

impl Iterator for Iter {
    type Item = *mut Node;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let head = self.current;
            if (*head).prev.is_null() {
                None
            } else {
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
            size: 0,
            head: null_mut(),
        }
    }

    // pub fn iter_mut(&mut self) -> Iter {
    //     Iter { current: self.head }
    // }

    pub fn iter(&self) -> Iter {
        Iter { current: self.head }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        for _ in self.iter() {
            len += 1;
        }
        len
    }
    pub fn size(&self) -> usize{
        self.size
    }
    pub unsafe fn init(&mut self, start: usize, end: usize, chunk_size: usize) {
        info!("algin before end: {}", end);
        let start = align_to_up(start, chunk_size);
        let end = align_to_down(end, chunk_size);
        self.size = chunk_size;
        info!(
            "start: {:#x} end: {:#x} chunk_size: {:#x}",
            start, end, chunk_size
        );
        for address in (start..end).step_by(chunk_size) {
            let head = Node::to_current(address);
            if address + chunk_size == end {
                unsafe { (*head).prev = null_mut() }
            }
            let prev = Node::to_prev(address + chunk_size);
            unsafe {
                (*head).prev = prev;
            }
        }
        self.head = Node::to_current(start);
        info!("init ok the len is {}",self.len())
    }

    pub unsafe fn alloc<T>(&mut self) -> Option<*mut T> {
        info!("alloc ");
        if self.head.is_null() {
            return None;
        }
        let head = self.head;
        self.head = (*head).prev;
        Some(head as *mut T)
    }

    pub unsafe fn dealloc(&mut self, address: usize) {
        info!("now dealloc");
        let head = Node::to_current(address);
        assert!(!head.is_null());
        (*head).prev = self.head;
        self.head = head;
    }

    pub unsafe fn dealloc_tail(&mut self, address: usize) {
        let tail = Node::to_current(address);
        (*tail).prev = null_mut();
        assert!(!tail.is_null());
        let mut current = self.head;
        while !(*(self.head)).prev.is_null() {
            current = (*current).prev
        }
        (*current).prev = tail;
    }
}

#[cfg(test)]
mod tests {
    use xxos_log::{info, init_log};

    use super::*;
    use crate::linklist::def::POLL_64;
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
            init_log(&PT);
            let start = &heap as *const _ as usize;
            let end = &heap[4095] as *const _ as usize;
            std::println!("size {:#x} len {}", (end - start), (end - start) / 64);
            let mut linked_list = Linkedlist::new();
            linked_list.init(start, end, POLL_64);
            info!("len of linkedlist: {}", linked_list.len())
        }
    }
}
