//! Prints the user LED of the Nucleo Board
//! ---

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32f1xx_hal::{delay::Delay, pac, prelude::*}; // STM32F1 specific functions
#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp =  cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let mut gpio_a = dp.GPIOA.split();
    let mut gpio_c = dp.GPIOC.split();

    let mut led = gpio_a.pa5.into_push_pull_output(&mut gpio_a.crl);
    let button = gpio_c.pc13.into_floating_input(&mut gpio_c.crh);

    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut led_state = false;
    let mut prev_button_state = button.is_high();
    loop {
        let curr_button_state = button.is_high();
        // Detect rising edge: prev low, curr high
        if !prev_button_state && curr_button_state {
            led_state = !led_state;
            if led_state {
                led.set_high();
            } else {
                led.set_low();
            }
        }
        prev_button_state = curr_button_state;
        delay.delay_ms(10_u16); // debounce and poll interval
    }
}

