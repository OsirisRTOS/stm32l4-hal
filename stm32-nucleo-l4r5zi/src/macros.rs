#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(concat!($s, "\0").as_bytes()) }
    };
}
