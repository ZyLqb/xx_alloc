use super::tree_node::TreeNode;

#[allow(dead_code)]
pub struct BinTree {
    root: Option<*const TreeNode>,
}

#[allow(dead_code)]
impl BinTree {
    pub fn new(root: usize) -> Self {
        let root = TreeNode::new(root);
        Self {
            root: Some(root.as_ptr()),
        }
    }

    pub fn find(&self, val: usize) -> *const TreeNode {
        let _ = val;
        todo!()
    }
}

#[cfg(test)]
pub mod tests {}
