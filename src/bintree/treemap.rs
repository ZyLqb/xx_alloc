use super::def::MAX_ARRAY;

pub struct TreeMap([u8; MAX_ARRAY]);

impl Default for TreeMap {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeMap {
    pub fn new() -> Self {
        Self([0; MAX_ARRAY])
    }

    pub fn get_bit(&self, idx: usize) -> bool {
        let byte_index = idx / 8;
        let bit_index = idx % 8;

        (self.0[byte_index] & (1 << bit_index)) != 0
    }

    pub fn set_bit(&mut self, idx: usize) {
        if !self.get_bit(idx) {
            let byte_index = idx / 8;
            let bit_index = idx % 8;

            self.0[byte_index] |= 1 << bit_index;
        }
    }

    pub fn unset_bit(&mut self, idx: usize) {
        if self.get_bit(idx) {
            let byte_index = idx / 8;
            let bit_index = idx % 8;

            self.0[byte_index] &= !(1 << bit_index);
        }
    }

    pub fn set_bit_all(&mut self) {
        for i in self.0.iter_mut() {
            *i = !0;
        }
    }

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
    use crate::bintree::def::*;
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
