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
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PA5, Level::High, Speed::Low); 

    // Set up DHT11 temperature and humidity sensor
    let mut dht11_data = Flex::new(p.PA0);
    dht11_data.set_as_input_output(Speed::Low);
    // let mut dht11 = Dht11::new(dht11_data);

    let mut delay = embassy_time::Delay;

    loop {
        dht11_data.set_low();
        delay.delay_ms(20);

        // Restore floating
        dht11_data.set_high();
        delay.delay_us(30);
        dht11_data.set_low();
        delay.delay_us(60);
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
