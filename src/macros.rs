#[macro_export]
macro_rules! align {
    ($size:expr, $mask:expr) => {
        ($size & $mask)
    };
}

#[macro_export]
macro_rules! align_down {
    ($address:expr, $align_size:expr) => {
        ($crate::align!(($address), !($align_size - 1)))
    };
}

#[macro_export]
macro_rules! align_up {
    ($address:expr, $align_size:expr) => {
        (($crate::align!(($address), !($align_size - 1))) + ($align_size))
    };
}

#[macro_export]
macro_rules! is_align {
    ($size:expr, $align_size:expr) => {
        ($size & ($align_size - 1)) == 0
    };
}

#[cfg(test)]
pub mod macro_test {
    extern crate std;

    #[test]
    fn align_test() {
        assert_eq!(0x810, align!(0x810, !0xf));
        assert_eq!(0x810, align!(0x818, !0xf));
        assert_eq!(0x810, align_down!(0x818, 0x10));
        assert_eq!(0x820, align_up!(0x818, 0x10));
    }
}
