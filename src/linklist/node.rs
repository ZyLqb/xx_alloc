use core::{ptr::{NonNull, null_mut}, ops::Deref};
#[derive(Debug,Clone)]
#[repr(transparent)]
pub struct Node {
    pub prev: *mut Node
}

impl Node {
    pub fn new() -> Self{
        Self{
            prev: null_mut()
        }
    }
    pub fn to_current(address: usize) -> *mut Self {
        address as *mut Self
    }

    pub fn set_prev(&mut self,next: *mut Node){
        self.prev = next
    }

    pub fn to_prev(address: usize) -> *mut Self{
        address as *mut Self
    }
}