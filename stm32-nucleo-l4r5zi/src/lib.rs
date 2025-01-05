#![no_std]

pub mod macros;
pub mod semih;
pub mod gpio;

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
pub extern "C" fn hal_semih_write(msg: *const i8) -> u32 {
    semih::write(unsafe { core::ffi::CStr::from_ptr(msg) })
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
