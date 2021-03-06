//! A blinky-LED example application
//! This example uses launchpad-rs.

#![no_std]
#![no_main]
#![feature(alloc, collections, asm)]
#![crate_type = "staticlib"]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate alloc;
extern crate embedded_hal;
extern crate stellaris_launchpad;

use core::fmt::Write;
use stellaris_launchpad::board;
use stellaris_launchpad::cpu::{systick, timer, uart};
use embedded_hal::serial::Read as ReadHal;

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

#[no_mangle]
pub extern "C" fn main(launchpad: &mut board::Board) {
    let mut uart = uart::Uart::new(
        uart::UartId::Uart0,
        115200,
        uart::NewlineMode::SwapLFtoCRLF,
        launchpad.pins.pa0.take().unwrap(),
        launchpad.pins.pa1.take().unwrap(),
    );
    let mut loops = 0;
    let mut ticks_last = systick::SYSTICK_MAX;
    let mut t = timer::Timer::new(timer::TimerId::Timer1A);
    t.enable_pwm(4096);
    if let &mut Some(ref mut led) = &mut launchpad.leds.blue {
        led.enable_pwm();
    }
    let levels = [1u32, 256, 512, 1024, 2048, 4096];
    uart.write_all("Welcome to Launchpad Blink\n");
    loop {
        for level in levels.iter() {
            t.set_pwm(*level);
            let delta = systick::get_since(ticks_last);
            ticks_last = systick::get_ticks();
            writeln!(
                uart,
                "Hello, world! Loops = {}, elapsed = {}, run_time = {}, level = {}",
                loops,
                systick::ticks_to_usecs(delta),
                systick::run_time_us() as u32,
                level
            ).unwrap();
            while let Ok(ch) = uart.read() {
                writeln!(uart, "byte read {}", ch).unwrap();
            }
            loops = loops + 1;
            stellaris_launchpad::delay(250);
        }
    }
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
