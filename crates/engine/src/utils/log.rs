#[cfg(feature = "dbg-assert")]
#[macro_export]
macro_rules! dbg_assert {
    ($($arg:tt)*) => ( assert!($($arg)*); )
}


#[cfg(not(feature = "dbg-assert"))]
#[macro_export]
macro_rules! dbg_assert {
    ($($arg:tt)*) => {};
}
