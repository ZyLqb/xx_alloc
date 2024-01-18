#[macro_export]
macro_rules! align_addr {
    ($size:expr, $mask:expr) => {
        ($size & $mask)
    };
}
