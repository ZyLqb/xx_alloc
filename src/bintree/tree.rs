use crate::align_down;

use super::{def::*, treemap::TreeMap};

pub struct BinTree {
    nodes: [usize; MAX_NODES],
    bitmap: TreeMap,
    level: usize,
}

#[allow(dead_code)]
impl BinTree {
    pub fn new() -> Self {
        Self {
            nodes: [0; MAX_NODES],
            bitmap: TreeMap::new(),
            level: 0,
        }
    }

    pub fn generate(&mut self, root: usize, size: usize) -> Result<usize, &str> {
        let mem_size = align_down!(size, MIN_SIZE);
        let mem_counts = mem_size / MIN_SIZE;

        for i in self.nodes.iter_mut() {
            (*i) = 0;
        }
        self.bitmap.set_bit_all();
        self.level = 0;

        if mem_counts > 0 {
            let mut cur_size = mem_size;
            let mut counts = 0;

            while counts <= mem_counts {
                let mut current = root;

                while current < (root + mem_size) {
                    self.nodes[counts] = current;
                    self.bitmap.unset_bit(counts);

                    current += cur_size;
                    counts += 1;
                }

                cur_size >>= 1;
                self.level += 1;
            }

            Ok(mem_counts)
        } else {
            Err("have wrong in generate")
        }
    }

    fn get_level(&self, size: usize) -> usize {
        let mut index_size = size & !0xf;
        let mut level = self.level;

        while index_size >= MIN_SIZE {
            index_size >>= 1;
            level -= 1;
        }

        level
    }

    fn get_index(&self, level: usize) -> usize {
        2usize.pow(level as u32) - 1
    }

    pub fn find(&self, size: usize) -> Result<usize, &str> {
        let level = self.get_level(size);

        for i in (self.get_index(level))..(self.get_index(level + 1)) {
            if !self.bitmap.get_bit(i) {
                return Ok(i);
            }
        }

        Err("BinTree::find")
    }
}

#[cfg(test)]
pub mod tests {
    use super::BinTree;
    use crate::linklist::def::PGSZ;

    extern crate alloc;
    extern crate std;
    use std::panic;

    #[test]
    fn get_level_test() {
        let mut tree1 = BinTree::new();
        let mut tree2 = BinTree::new();
        let mut tree3 = BinTree::new();
        let _ = tree1.generate(0x10000, PGSZ);
        let _ = tree2.generate(0x10000, PGSZ / 2);
        let _ = tree3.generate(0x10000, PGSZ / 3);

        for i in 0..tree1.level {
            // println!("{}", tree1.get_level(PGSZ / (1 << i)));
            assert_eq!(i, tree1.get_level(PGSZ / (1 << i)));
        }

        for i in 0..tree2.level {
            // println!("{}", tree2.get_level(PGSZ / (2 << i)));
            assert_eq!(i, tree2.get_level(PGSZ / (2 << i)));
        }

        for i in 0..tree3.level {
            // println!("{}", tree3.get_level(PGSZ / (3 << i)));
            assert_eq!(i, tree3.get_level(PGSZ / (3 << i)));
        }
    }

    #[test]
    fn get_index_test() {
        let mut tree1 = BinTree::new();
        let mut tree2 = BinTree::new();
        let mut tree3 = BinTree::new();
        let _ = tree1.generate(0x10000, PGSZ);
        let _ = tree2.generate(0x10000, PGSZ / 2);
        let _ = tree3.generate(0x10000, PGSZ / 3);

        for i in 0..tree1.level {
            // println!("{}", tree1.get_index(level));
            assert_eq!((2usize.pow(i as u32)) - 1, tree1.get_index(i));
        }

        for i in 0..tree2.level {
            // println!("{}", tree2.get_index(level));
            assert_eq!((2usize.pow(i as u32)) - 1, tree2.get_index(i));
        }

        for i in 0..tree3.level {
            // println!("{}", tree3.get_index(level));
            assert_eq!((2usize.pow(i as u32)) - 1, tree3.get_index(i));
        }
    }

    #[test]
    fn find_test() {
        let mut tree = BinTree::new();
        let _ = tree.generate(0x10000, PGSZ);

        match tree.find(PGSZ) {
            Ok(idx) => assert_eq!(0, idx),
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[test]
    fn generate_test() {
        let mut tree = BinTree::new();
        let mut bad_tree = BinTree::new();

        let gen_success = tree.generate(0x10000, PGSZ);
        let gen_error = bad_tree.generate(0x10000, 0xf);

        assert!(gen_error.is_err());
        assert!(gen_success.is_ok());

        let root = tree.nodes[0];

        for level in 0..tree.level {
            let mut idx = tree.get_index(level);
            for i in (root..(root + PGSZ)).step_by(PGSZ >> level) {
                assert_eq!(i, tree.nodes[idx]);
                idx += 1;
            }
        }
    }
}
