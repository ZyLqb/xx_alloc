use crate::linklist::def::PGSZ;

pub(crate) const MAX_SIZE: usize = PGSZ << 10;
pub(crate) const MIN_SIZE: usize = PGSZ;
pub(crate) const MAX_LEAF: usize = MAX_SIZE / MIN_SIZE;
pub(crate) const MAX_PAGES: usize = MAX_LEAF * 2 - 1;
pub(crate) const MAX_ARRAY: usize = (MAX_PAGES + 1) / 8;
