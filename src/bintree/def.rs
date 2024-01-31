use crate::linklist::def::PGSZ;

pub(crate) const MAX_SIZE: usize = PGSZ << 10; // 可管理的最大内存
pub(crate) const MIN_SIZE: usize = PGSZ; // 可管理的最小内存
pub(crate) const MAX_LEAF: usize = MAX_SIZE / MIN_SIZE; // 树最大叶节点数
pub(crate) const MAX_NODES: usize = MAX_LEAF * 2 - 1; // 树的总节点数
