//! Blinky program for the STM32F767 using only _basic_ embedded Rust crates,
//! meaning no HAL is used and registers are modified by using the PAC.

#![no_std]
#![no_main]

// This crate has startup code and minimal runtime for Cortex-M microcontrollers
use cortex_m_rt::entry;

// This crate is used to access Cortex-M peripherals
use cortex_m::peripheral::syst::SystClkSource;

// This crate is used to access STM32 peripherals
use stm32f7::stm32f7x7;

// Used for logging over the debugger, integrated into "cargo embed"
use rtt_target::{rprintln, rtt_init, set_print_channel};

// All Rust programs need a panic handler, this crate sends panics over RTT
extern crate panic_rtt_target;

#[entry]
fn main() -> ! {
    // Setup RTT for logging
    let channels = rtt_init! {
        up: {
            0: {
                size: 4096
                mode: NoBlockSkip
                name: "Terminal"
            }
        }
    };

    set_print_channel(channels.up.0);

    // Get peripherals
    let cm_periph = cortex_m::Peripherals::take().unwrap();
    let pac_periph = stm32f7x7::Peripherals::take().unwrap();

    // Configure SysTick to one second (16 MHz HSI is default clock)
    let mut systick = cm_periph.SYST;
    systick.set_clock_source(SystClkSource::Core);
    systick.set_reload(16_000_000);
    systick.enable_counter();

    // Enable LED (GPIO B.7 on Nucleo): Enable clock gate, set pin output mode
    let rcc = pac_periph.RCC;
    rcc.ahb1enr.write(|w| w.gpioben().set_bit());

    let gpiob = pac_periph.GPIOB;
    gpiob.moder.write(|w| w.moder7().bits(0b01));

    loop {
        // Print to RTT
        rprintln!("Hello World!");

        // Toggle LED
        if gpiob.odr.read().odr7().is_low() {
            gpiob.odr.write(|w| w.odr7().set_bit());
        } else {
            gpiob.odr.write(|w| w.odr7().clear_bit());
        }

        // Delay for 1 second
        while !systick.has_wrapped() {}
    }
}
