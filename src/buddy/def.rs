use crate::bintree::def::MAX_LEAF;
use crate::def::PGSZ;

pub(crate) const PAGE_SIZE: usize = PGSZ;
pub(crate) const MAX_PAGES: usize = MAX_LEAF;

pub(crate) type MemPtr = usize;
