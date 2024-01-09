#[inline]
pub(crate) const fn align_to_down(address: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        address & !(align - 1)
    } else if align == 0 {
        address
    } else {
        panic!("error 'align must be power of 2 '")
    }
}
#[inline]
pub(crate) const fn align_to_up(address: usize, align: usize) -> usize {
    align_to_down(address + align - 1, align)
}
