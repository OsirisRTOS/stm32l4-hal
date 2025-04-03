#![no_std]

pub mod gpio;
pub mod macros;
pub mod semih;

pub extern crate cortex_m as common;

use common::*;
use core::ffi::c_char;
use core::ops::Add;

const RCC_BASE: u32 = 0x40021000;
const RCC_AHB2ENR_OFFSET: u32 = 0x4C;

#[unsafe(no_mangle)]
pub extern "C" fn default_hndlr() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn hal_hw_init() {
    //enable GPIO peripheral clock
    let ptr: *mut u32 = RCC_BASE.add(RCC_AHB2ENR_OFFSET) as *mut u32;
    //SAFETY:
    // This is a write to a known memory address which is a memory mapped register
    unsafe {
        ptr.write_volatile(0x000001ff);
    }

    init_systick();
}

fn init_systick() {
    let mut systick = peripheral::SYST::new();
    let reload = peripheral::SYST::get_ticks_per_10ms();

    systick.enable_interrupt();
    systick.set_reload(reload);
    systick.clear_current();
    systick.enable_counter();
}

/*
 * Function: hal_semih_write
 * Usage: hal_semih_write("Hello, world!\n");
 * Preconditions: msg is a valid null terminated UTF-8 C string.
 * Postconditions: none
 */
#[unsafe(no_mangle)]
pub extern "C" fn hal_semih_write_debug(msg: *const c_char) {
    cortex_m::semih::hio::write_debug(unsafe { core::ffi::CStr::from_ptr(msg) });
}
