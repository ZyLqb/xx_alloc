#[macro_export]
macro_rules! align {
    ($size:expr, $mask:expr) => {
        ($size & $mask)
    };
}

#[macro_export]
macro_rules! is_align {
    ($size:expr, $align_size:expr) => {
        ($size & ($align_size - 1)) == 0
    };
}

#[macro_export]
macro_rules! align_down {
    ($address:expr, $align_size:expr) => {
        if ($crate::is_align!(($address), ($align_size))) {
            ($address)
        } else {
            ($crate::align!(($address), !($align_size - 1)))
        }
    };
}

#[macro_export]
macro_rules! align_up {
    ($address:expr, $align_size:expr) => {
        if ($crate::is_align!(($address), ($align_size))) {
            ($address)
        } else {
            (($crate::align!(($address), !($align_size - 1))) + ($align_size))
        }
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
        assert_eq!(0x820, align_up!(0x820, 0x10));

        assert_eq!(0x7fc000, align_up!(0x7fb078, 0x2000));
        assert_eq!(0x7fa000, align_down!(0x7fb078, 0x2000));
    }
}
