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
            bitmap: TreeMap::new(true),
            level: 0,
        }
    }

    pub fn generate(&mut self, root: usize, size: usize) -> bool {
        let mem_size = size & (!MIN_SIZE);
        let mem_counts = mem_size / MIN_SIZE;

        if mem_counts > 0 {
            let mut cur_size = mem_size;
            let mut counts = 0;

            while counts <= mem_counts {
                let mut current = root;

                while current < (root + mem_size) {
                    self.nodes[counts] = current;

                    counts += 1;
                    current += cur_size;
                    self.bitmap.unset_bit(counts);
                }
                cur_size >>= 1;
                self.level += 1;
            }

            true
        } else {
            false
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

    // FIXME
    pub fn find(&self, size: usize) -> Result<usize, &str> {
        let level = self.get_level(size);

        for i in (self.get_index(level))..(self.get_index(level + 1) - 1) {
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
    use crate::{bintree::def::MIN_SIZE, linklist::def::PGSZ};

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
            let level = PGSZ / (1 << i);
            // println!("{}", tree1.get_index(level));
            assert_eq!((2usize.pow(i as u32)) - 1, tree1.get_index(level));
        }

        for i in 0..tree2.level {
            let level = PGSZ / (2 << i);
            // println!("{}", tree2.get_index(level));
            assert_eq!((2usize.pow(i as u32)) - 1, tree2.get_index(level));
        }

        for i in 0..tree3.level {
            let level = PGSZ / (3 << i);
            // println!("{}", tree3.get_index(level));
            assert_eq!((2usize.pow(i as u32)) - 1, tree3.get_index(level));
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

        if gen_error || !gen_success {
            panic!();
        }

        let root = tree.nodes[0];
        let level = tree.get_level(MIN_SIZE);
        //for i in (root..((root + PGSZ) & !1)).step_by(MIN_SIZE) {}
        //for i in (root..((root + PGSZ) & !1)).step_by(MIN_SIZE * (1 << 1)) {}
        //for i in (root..((root + PGSZ) & !1)).step_by(MIN_SIZE * (1 << 2)) {}
        //for i in (root..((root + PGSZ) & !1)).step_by(MIN_SIZE * (1 << 3)) {}
        //for i in (root..((root + PGSZ) & !1)).step_by(MIN_SIZE * (1 << 4)) {}
        //for i in (root..((root + PGSZ) & !1)).step_by(MIN_SIZE * (1 << 5)) {}
        //for i in (root..((root + PGSZ) & !1)).step_by(MIN_SIZE * (1 << 6)) {}
        //for i in (root..((root + PGSZ) & !1)).step_by(MIN_SIZE * (1 << 7)) {}
    }
}
