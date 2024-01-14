#[allow(dead_code)]
pub struct TreeNode {
    value: usize,
    visited: bool,
    left: Option<*const TreeNode>,
    right: Option<*const TreeNode>,
}

#[allow(dead_code)]
impl TreeNode {
    pub fn new(val: usize) -> Self {
        Self {
            value: val,
            visited: false,
            left: None,
            right: None,
        }
    }

    pub fn set_value(&mut self, val: usize) -> usize {
        self.value = val;
        self.value
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn visit(&mut self) {
        self.visited = true;
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn as_ptr(&self) -> *const TreeNode {
        self as *const TreeNode
    }
}

#[allow(dead_code)]
impl TreeNode {
    fn insert_left(&mut self, val: usize) -> Result<usize, &str> {
        match self.left {
            Some(_) => Err("falut occured in insert_left."),
            None => {
                let left = TreeNode::new(val);
                self.left = Some(left.as_ptr());
                Ok(val)
            }
        }
    }

    fn insert_right(&mut self, val: usize) -> Result<usize, &str> {
        match self.right {
            Some(_) => Err("falut occured in insert_right."),
            None => {
                let right = TreeNode::new(val);
                self.right = Some(right.as_ptr());
                Ok(val)
            }
        }
    }

    fn delete_left(&mut self) -> Result<usize, &str> {
        match self.left {
            Some(left_leaf) => {
                self.left = None;
                unsafe {
                    let val = left_leaf.read().value;
                    Ok(val)
                }
            }
            None => Err("falut occured in delete_left."),
        }
    }

    fn delete_right(&mut self) -> Result<usize, &str> {
        match self.right {
            Some(right_leaf) => {
                self.right = None;
                unsafe {
                    let val = right_leaf.read().value;
                    Ok(val)
                }
            }
            None => Err("falut occured in delete_right."),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::TreeNode;

    #[test]
    fn insert_test() {
        let mut root = TreeNode::new(0);

        match root.insert_left(1) {
            Ok(_) => {}
            Err(err) => {
                panic!("{}", err);
            }
        }

        match root.insert_right(2) {
            Ok(_) => {}
            Err(err) => {
                panic!("{}", err);
            }
        }

        unsafe {
            assert_eq!(0, root.value);
            assert_eq!(1, root.left.unwrap().read().value);
            assert_eq!(2, root.right.unwrap().read().value);
        }
    }

    #[test]
    fn delete_test() {
        let mut root = TreeNode::new(0);

        let _ = root.insert_left(1);
        let _ = root.insert_right(2);

        assert_eq!(1, root.delete_left().unwrap());
        assert_eq!(2, root.delete_right().unwrap());
    }
}
