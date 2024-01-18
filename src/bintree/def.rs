use crate::linklist::def::PGSZ;

pub(crate) const MAX_SIZE: usize = PGSZ;
pub(crate) const MIN_SIZE: usize = 1 << 4;
pub(crate) const MAX_LEAF: usize = MAX_SIZE / MIN_SIZE;
pub(crate) const MAX_NODES: usize = MAX_LEAF * 2 - 1;
pub(crate) const MAX_ARRAY: usize = (MAX_NODES + 1) / 8;
