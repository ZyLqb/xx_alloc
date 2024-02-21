use core::mem::size_of;

use super::def::MAX_NODES;

// 二叉树的位图
#[derive(Debug)]
#[repr(C)]
pub struct TreeMap([u8; MAX_NODES]);

impl Default for TreeMap {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused)]
impl TreeMap {
    pub fn new() -> Self {
        Self([0; MAX_NODES])
    }

    // 获取对应bit位
    pub fn is_empty(&self, idx: usize) -> bool {
        let byte_index = idx / size_of::<usize>();
        let bit_index = idx % size_of::<usize>();

        (self.0[byte_index] & (1 << bit_index)) == 0
    }

    // 设置对应bit位为1
    pub fn set_bit(&mut self, idx: usize) {
        let byte_index = idx / size_of::<usize>();
        let bit_index = idx % size_of::<usize>();

        self.0[byte_index] |= 1 << bit_index;
    }

    // 设置对应bit位为0
    pub fn unset_bit(&mut self, idx: usize) {
        let byte_index = idx / size_of::<usize>();
        let bit_index = idx % size_of::<usize>();

        self.0[byte_index] &= !(1 << bit_index);
    }

    // 设置全部bit位为1
    pub fn set_bit_all(&mut self) {
        for i in self.0.iter_mut() {
            *i = !0;
        }
    }

    // 设置全部bit位为0
    pub fn unset_bit_all(&mut self) {
        for i in self.0.iter_mut() {
            *i = 0;
        }
    }
}

#[cfg(test)]
pub mod tests {
    extern crate std;
    use super::TreeMap;
    use crate::bintree::def::MAX_NODES;
    use std::panic;

    #[test]
    fn map_test() {
        let mut bitmap = TreeMap::new();

        for i in 0..MAX_NODES {
            if bitmap.is_empty(i) {
                bitmap.set_bit(i);
                assert!(!bitmap.is_empty(i));
                bitmap.unset_bit(i);
                assert!(bitmap.is_empty(i));
            } else {
                panic!();
            }
        }

        bitmap.set_bit_all();
        for i in 0..MAX_NODES {
            assert!(!bitmap.is_empty(i));
        }

        bitmap.unset_bit_all();
        for i in 0..MAX_NODES {
            assert!(bitmap.is_empty(i));
        }
    }
}
