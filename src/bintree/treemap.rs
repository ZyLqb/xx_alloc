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
    pub fn get_bit(&self, idx: usize) -> bool {
        let byte_index = idx / 8;
        let bit_index = idx % 8;

        (self.0[byte_index] & (1 << bit_index)) != 0
    }

    // 批量获取对应bit位，若全为1则为1，否则为0
    pub fn mul_get_bit(&self, index: usize, counts: usize) -> bool {
        for i in 0..counts {
            if self.get_bit(index + i) {
                return false;
            }
        }
        true
    }

    // 设置对应bit位为1
    pub fn set_bit(&mut self, idx: usize) {
        if !self.get_bit(idx) {
            let byte_index = idx / 8;
            let bit_index = idx % 8;

            self.0[byte_index] |= 1 << bit_index;
        }
    }

    // 设置对应bit位为0
    pub fn unset_bit(&mut self, idx: usize) {
        if self.get_bit(idx) {
            let byte_index = idx / 8;
            let bit_index = idx % 8;

            self.0[byte_index] &= !(1 << bit_index);
        }
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
            if !bitmap.get_bit(i) {
                bitmap.set_bit(i);
                assert!(bitmap.get_bit(i));
                bitmap.unset_bit(i);
                assert!(!bitmap.get_bit(i));
            } else {
                panic!();
            }
        }

        bitmap.set_bit_all();
        for i in 0..MAX_NODES {
            assert!(bitmap.get_bit(i));
        }

        bitmap.unset_bit_all();
        for i in 0..MAX_NODES {
            assert!(!bitmap.get_bit(i));
        }
    }
}
