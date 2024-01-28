#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Node {
    pub next: *mut Node,
}

impl Node {
    #[allow(unused)]
    pub fn set_next(&mut self, next: *mut Node) {
        self.next = next;
    }

    pub fn to_mut_node_ptr(address: usize) -> *mut Self {
        address as *mut Self
    }
}
