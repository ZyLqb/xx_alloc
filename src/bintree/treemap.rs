use super::def::MAX_ARRAY;

pub struct TreeMap([u8; MAX_ARRAY]);

#[allow(dead_code)]
impl TreeMap {
    pub fn new(bit: bool) -> Self {
        if bit {
            Self([0xff; MAX_ARRAY])
        } else {
            Self([0; MAX_ARRAY])
        }
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
}

#[cfg(test)]
pub mod tests {
    extern crate std;
    use super::TreeMap;
    use crate::bintree::def::*;
    use std::panic;

    #[test]
    fn map_test() {
        let mut bitmap_empty = TreeMap::new(false);
        let mut bitmap_full = TreeMap::new(true);

        for i in 0..MAX_NODES {
            if !bitmap_empty.get_bit(i) {
                bitmap_empty.set_bit(i);
                assert!(bitmap_empty.get_bit(i));
                bitmap_empty.unset_bit(i);
                assert!(!bitmap_empty.get_bit(i));
            } else {
                panic!();
            }
        }
        for i in 0..MAX_NODES {
            if bitmap_full.get_bit(i) {
                bitmap_full.unset_bit(i);
                assert!(!bitmap_full.get_bit(i));
                bitmap_full.set_bit(i);
                assert!(bitmap_full.get_bit(i));
            } else {
                panic!();
            }
        }
    }
}
