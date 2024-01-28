use super::{def::*, treemap::TreeMap};
use crate::{align_down, is_align};

/// complete binary tree
#[repr(C)]
#[derive(Debug)]
pub struct BinTree {
    pub level: usize,
    nodes: [usize; MAX_PAGES],
    pub bitmap: TreeMap,
}

#[allow(unused)]
impl BinTree {
    pub fn new() -> Self {
        Self {
            nodes: [0; MAX_PAGES],
            bitmap: TreeMap::new(),
            level: 0,
        }
    }

    pub fn init(&mut self, root: usize, size: usize) -> Result<usize, &str> {
        let mut mem_size = align_down!(size, MIN_SIZE);
        let mut page_counts = mem_size / MIN_SIZE;

        while page_counts > 0 && !page_counts.is_power_of_two() {
            page_counts -= 1;
            mem_size -= MIN_SIZE;
        }

        self.bitmap.set_bit_all();

        if page_counts == 0 {
            return Err("BinTree::init");
        }

        let node_counts = page_counts * 2 - 1;
        let mut cur_size = mem_size;
        let mut counts = 0;

        while counts < node_counts {
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

        Ok(page_counts)
    }

    pub fn get_level(&self, size: usize) -> usize {
        let mut index_size = align_down!(size, MIN_SIZE);
        let mut level = self.level;

        while index_size > MIN_SIZE {
            index_size >>= 1;
            level -= 1;
        }

        level
    }

    pub fn get_index(&self, level: usize) -> usize {
        2usize.pow((level - 1) as u32) - 1
    }

    pub fn get_value(&self, idx: usize) -> usize {
        self.nodes[idx]
    }

    // 进行适配搜索
    pub fn find_unused(&self, size: usize) -> Result<usize, &str> {
        if size > MAX_SIZE {
            return Err("BinTree::find");
        }

        let level = self.get_level(size);

        // verify bitmap
        let mut idx = self.get_index(level);
        while idx < (self.get_index(level + 1) - 1) {
            if !self.bitmap.get_bit(idx) {
                let mut left_leaf = idx;

                while self.find_left_child(left_leaf) <= self.max_node() {
                    left_leaf = self.find_left_child(left_leaf);
                }

                let mut page_counts = size / MIN_SIZE;
                let mut page = 0;
                if self.bitmap.mul_get_bit(left_leaf, page_counts) {
                    break;
                } else {
                    idx += 1;
                }
            } else {
                idx += 1;
            }
        }
        Ok(idx)
    }

    pub fn find_used(&self, size: usize) -> Result<usize, &str> {
        if size > MAX_SIZE {
            return Err("BinTree::find");
        }

        let level = self.get_level(size);

        // verify bitmap
        let mut idx = self.get_index(level);
        let max_idx = self.get_index(level + 1) - 1;
        while idx < max_idx {
            if self.bitmap.get_bit(idx) {
                break;
            }

            idx += 1;
        }

        Ok(idx)
    }

    pub fn max_node(&self) -> usize {
        self.get_index(self.level + 1) - 1
    }

    pub fn use_mem(&mut self, idx: usize) {
        let mut left_leaf = idx;
        let mut level = 0;

        while left_leaf <= self.max_node() {
            for i in 0..2usize.pow(level) {
                self.bitmap.set_bit(left_leaf + i);
            }

            left_leaf = self.find_left_child(left_leaf);
            level += 1;
        }
    }

    pub fn unuse_mem(&mut self, idx: usize) {
        let mut left_leaf = idx;
        let mut level = 0;

        while left_leaf <= self.max_node() {
            for i in 0..2usize.pow(level) {
                self.bitmap.unset_bit(left_leaf + i);
            }

            left_leaf = self.find_left_child(left_leaf);
            level += 1;
        }
    }

    pub fn use_page(&mut self, idx: usize) {
        self.bitmap.set_bit(idx);
    }

    pub fn unuse_page(&mut self, idx: usize) {
        self.bitmap.unset_bit(idx);
    }

    pub fn find_left_child(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    pub fn find_right_child(&self, idx: usize) -> usize {
        idx * 2 + 2
    }

    pub fn find_parent(&self, idx: usize) -> usize {
        (idx + 1) / 2 - 1
    }
}

#[cfg(test)]
pub mod tests {
    use super::BinTree;
    use crate::linklist::def::PGSZ;
    extern crate alloc;
    extern crate std;
    use std::{panic, println};
    use xxos_log::LOG;
    use xxos_log::{info, init_log, WriteLog};
    struct PT;

    impl WriteLog for PT {
        fn print(&self, log_content: core::fmt::Arguments) {
            println!("{}", log_content);
        }
    }

    #[test]
    fn get_level_test() {
        let mut tree1 = BinTree::new();
        let mut tree2 = BinTree::new();
        let mut tree3 = BinTree::new();
        let _ = tree1.init(0x10000, PGSZ);
        let _ = tree2.init(0x10000, PGSZ * 2);
        let _ = tree3.init(0x10000, PGSZ * 3);

        for i in 0..tree1.level {
            assert_eq!(i + 1, tree1.get_level(PGSZ * (1 >> i)));
        }

        for i in 0..tree2.level {
            assert_eq!(i + 1, tree2.get_level(PGSZ * (2 >> i)));
        }

        for i in 0..tree3.level {
            assert_eq!(i + 1, tree3.get_level(PGSZ * (2 >> i)));
        }
    }

    #[test]
    fn get_index_test() {
        let mut tree1 = BinTree::new();
        let mut tree2 = BinTree::new();
        let mut tree3 = BinTree::new();
        let _ = tree1.init(0x10000, PGSZ);
        let _ = tree2.init(0x10000, PGSZ * 2);
        let _ = tree3.init(0x10000, PGSZ * 3);

        for i in 0..tree1.level {
            assert_eq!((2usize.pow(i as u32)) - 1, tree1.get_index(i + 1));
        }

        for i in 0..tree2.level {
            assert_eq!((2usize.pow(i as u32)) - 1, tree2.get_index(i + 1));
        }

        for i in 0..tree3.level {
            assert_eq!((2usize.pow(i as u32)) - 1, tree3.get_index(i + 1));
        }
    }

    #[test]
    fn find_test() {
        let mut tree = BinTree::new();
        let _ = tree.init(0x10000, PGSZ);

        match tree.find_unused(PGSZ) {
            Ok(idx) => assert_eq!(0, idx),
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[test]
    fn init_test() {
        init_log(&PT, xxos_log::Level::INFO);

        let mut tree = BinTree::new();
        let mut bad_tree = BinTree::new();

        let gen_success = tree.init(0x10000, PGSZ * 10);
        let gen_error = bad_tree.init(0x10000, PGSZ / 2);

        assert_eq!(Ok(8), gen_success);
        assert!(gen_error.is_err());

        info!("{:x?}", tree);
    }
}
