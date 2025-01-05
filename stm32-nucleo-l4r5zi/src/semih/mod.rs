pub mod macros;

use core::ffi::CStr;

pub fn write_debug(msg: impl AsRef<CStr>) {
    common::semih::hio::write_debug(msg.as_ref());
}
