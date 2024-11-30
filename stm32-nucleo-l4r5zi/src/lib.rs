#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn default_hndlr() {
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
