use core::arch::asm;
use core::ffi::CStr;

pub fn write(msg: &CStr) -> u32 {
    let reason = 0x04;
    let mut result: u32 = 0;

    unsafe {
        asm!(
            "mov r0, {0}",
            "mov r1, {1}",
            "bkpt 0xAB",
            "mov {2}, r0",
            in(reg) reason,
            in(reg) msg.as_ptr(),
            out(reg) result
        );
    }

    return 0;
}
