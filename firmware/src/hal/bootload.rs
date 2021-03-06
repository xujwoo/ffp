// Copyright 2019 Adam Greig
// Dual licensed under the Apache 2.0 and MIT licenses.

use stm32ral::{write_reg, syscfg, scb};

static mut FLAG: u32 = 0;
const FLAG_VALUE: u32 = 0xB00110AD;

/// Call this function at boot in pre_init, before statics are initialised.
///
/// If we reset due to requesting a bootload, this function will jump to
/// the system bootloader.
pub unsafe fn check() {
    // If flag isn't set we just continue with the boot process
    if core::ptr::read_volatile(&FLAG) != FLAG_VALUE {
        return;
    }

    // Otherwise, clear the flag and jump to system bootloader
    core::ptr::write_volatile(&mut FLAG, 0);

    // Remap system memory to 0x0000_0000
    write_reg!(syscfg, SYSCFG, CFGR1, MEM_MODE: SystemFlash);

    // Get new stack pointer and jump address
    let sp = core::ptr::read_volatile(0 as *const u32);
    let rv = core::ptr::read_volatile(4 as *const u32);
    let bootloader: extern "C" fn() = core::mem::transmute(rv);

    // Write new stack pointer to MSP and call into system memory
    cortex_m::register::msp::write(sp);
    bootloader();
}

/// Call this function to trigger a reset into the system bootloader
pub fn bootload() -> ! {
    unsafe {
        // Write flag value to FLAG
        core::ptr::write_volatile(&mut FLAG, FLAG_VALUE);

        // Request system reset
        write_reg!(scb, SCB, AIRCR, VECTKEYSTAT: 0x05FA, SYSRESETREQ: 1);
    }

    // Wait for reset
    loop {
        cortex_m::asm::nop();
    }
}
