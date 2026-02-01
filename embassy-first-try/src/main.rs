#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_stm32;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed, Flex};
use embassy_time::{Duration, Timer};
use fmt::info;
use dht11::Dht11;

use embedded_hal::{
    delay::DelayNs
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();
    // Configure HSI -> PLL to the highest achievable system clock using HSI.
    // HSI is 8 MHz; PLL input is HSI/2 (4 MHz). Max PLL multiplication supported
    // lets us reach 64 MHz (4 MHz * 16). We'll request HSI + PLL x16 and set
    // typical prescalers (AHB = sys, APB1 = /2, APB2 = sys).
    use embassy_stm32::rcc as rcc_mod;
    // Configure PLL to use HSI/2 and multiply to reach 64 MHz (4 MHz * 16).
    config.rcc.pll = Some(rcc_mod::Pll {
        src: rcc_mod::PllSource::HSI,
        prediv: rcc_mod::PllPreDiv::DIV2,
        mul: rcc_mod::PllMul::MUL16,
    });
    // Request PLL as system clock and set prescalers.
    config.rcc.sys = rcc_mod::Sysclk::PLL1_P;
    config.rcc.ahb_pre = rcc_mod::AHBPrescaler::DIV1;
    config.rcc.apb1_pre = rcc_mod::APBPrescaler::DIV2;
    config.rcc.apb2_pre = rcc_mod::APBPrescaler::DIV1;

    let p = embassy_stm32::init(config);
    let mut led = Output::new(p.PA5, Level::High, Speed::Low); 

    // Set up DHT11 temperature and humidity sensor
    let mut dht11_data = Flex::new(p.PA0);
    dht11_data.set_as_input_output(Speed::VeryHigh);
    // let mut dht11 = Dht11::new(dht11_data);

    let mut delay = embassy_time::Delay;

    loop {
        dht11_data.set_low();
        delay.delay_ms(10);

        // Restore floating
        dht11_data.set_high();
        cortex_m::asm::delay(80); 
        // delay.delay_us(5);
        dht11_data.set_low();
        cortex_m::asm::delay(8); 
        // delay.delay_us(1);
        dht11_data.set_high();
        // let result = dht11.perform_measurement(&mut delay);
        // match result {
        //     Ok(measurement) => {
        //         info!("Temperature: {}°C, Humidity: {}%", measurement.temperature, measurement.humidity);
        //     }
        //     Err(_e) => {
        //         match _e {
        //             dht11::Error::Timeout => info!("DHT11 sensor timeout."),
        //             dht11::Error::CrcMismatch => info!("DHT11 checksum error."),
        //             _ => info!("DHT11 GPIO error."),
        //         }
        //     }
        // }
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}
