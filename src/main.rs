//! A blinky-LED example application
//! This example uses Primer, a library for simple bare-metal ARM programming.

#![feature(lang_items)]
#![no_std]
#![no_main]
#![crate_type="staticlib"]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate primer;

use primer::gpio;
use primer::launchpad;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

pub fn led_on() {
    gpio::set(launchpad::LED_RED, gpio::Level::High);
}

pub fn led_off() {
    gpio::set(launchpad::LED_RED, gpio::Level::Low);
}

#[no_mangle]
pub extern "C" fn rust_loop() {
    launchpad::init();
    loop {
        led_on();
        primer::delay(100);
        led_off();
        primer::delay(100);
    }
}

#[lang="eh_personality"]
extern "C" fn eh_personality() {

}

#[lang="panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_unwind(_fmt: &core::fmt::Arguments,
                                    _file_line: &(&'static str, usize))
                                    -> ! {
    loop {}
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
