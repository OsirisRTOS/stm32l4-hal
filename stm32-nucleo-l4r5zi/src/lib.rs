#![no_std]

pub mod macros;
pub mod semih;
pub mod gpio;
mod uart;

pub extern crate cortex_m as common;

use core::ops::Add;

const RCC_BASE:u32 = 0x40021000;
const RCC_AHB2ENR_OFFSET:u32 =  0x4C;

#[no_mangle]
pub extern "C" fn default_hndlr() {
    loop {}
}

#[no_mangle]
pub extern "C" fn hal_hw_init() {
    //enable GPIO peripheral clock
    let ptr:*mut u32 = RCC_BASE.add(RCC_AHB2ENR_OFFSET) as *mut u32;
    //SAFETY
    //This is a write to a known memory address which is a memory mapped register
    unsafe { ptr.write_volatile(0x000001ff); }
}

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
