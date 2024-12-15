#[macro_export]
macro_rules! hprintln {
    () => {
        $crate::common::hprintln!()
    };
    ($($arg:tt)*) => {
        $crate::common::hprintln!($($arg)*)
    };
}
