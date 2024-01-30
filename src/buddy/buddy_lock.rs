use super::{buddy_allocator::BuddyAllocator, def::MemPtr};
use xx_mutex_lock::Mutex;

pub struct LockedBuddy(Mutex<BuddyAllocator>);

unsafe impl Send for LockedBuddy {}

/// TODO
/// 页分配器多线程共享实现
#[allow(unused)]
impl LockedBuddy {
    pub fn new() -> Self {
        let heap = BuddyAllocator::new();
        LockedBuddy(Mutex::new(heap))
    }

    pub fn init(&self, bottom: MemPtr, top: MemPtr) {
        unsafe { self.0.lock().init(bottom, top) };
    }
}
