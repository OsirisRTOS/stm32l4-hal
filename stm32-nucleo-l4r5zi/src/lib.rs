#![no_std]

pub mod macros;
pub mod semih;

pub extern crate cortex_m as common;

#[no_mangle]
pub extern "C" fn default_hndlr() {
    loop {}
}

#[no_mangle]
pub extern "C" fn hal_hw_init() {}

/*
 * Function: hal_semih_write
 * Usage: hal_semih_write("Hello, world!\n");
 * Preconditions: msg is a valid null terminated UTF-8 C string.
 * Postconditions: none
 */
#[no_mangle]
pub extern "C" fn hal_semih_write_debug(msg: *const i8) {
    cortex_m::semih::hio::write_debug(unsafe { core::ffi::CStr::from_ptr(msg) });
}
