use super::{buddy_allocator::BuddyAllocator, def::MemPtr};
use xx_mutex_lock::Mutex;

pub struct LockedBuddy(Mutex<BuddyAllocator>);

unsafe impl Send for LockedBuddy {}

/// TODO
/// 页分配器多线程共享实现
#[allow(unused)]
impl LockedBuddy {
    pub fn new(bottom: MemPtr, top: MemPtr) -> Self {
        let heap = BuddyAllocator::new(bottom, top);
        LockedBuddy(Mutex::new(heap))
    }
}
