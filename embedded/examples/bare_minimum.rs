//! Bare minimum needed for an embedded Rust program on the STM32F767.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f7::stm32f7x7;

// All Rust programs need a panic handler
extern crate panic_halt;

#[entry]
fn main() -> ! {
   loop {}
}
