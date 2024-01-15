use crate::linklist::def::PGSZ;

pub const MAX_SIZE: usize = PGSZ;
pub const MIN_SIZE: usize = 1 << 4;
pub const MAX_LEAF: usize = MAX_SIZE / MIN_SIZE;
pub const MAX_NODES: usize = MAX_LEAF * 2 - 1;
pub const MAX_ARRAY: usize = (MAX_NODES + 1) / 8;
