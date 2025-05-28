#![no_std]
#![no_main]

use esp32_hal::{adc::Adc, clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};
use mq135::{GasType, Mq135};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = peripherals.GPIO.split();
    let adc = Adc::adc1(peripherals.SENS, peripherals.APB_SARADC, &clocks);
    let mut pin = io.p34.into_analog(); // adjust to your analog pin

    let mut mq = Mq135::new(adc, pin, 10.0);

    mq.calibrate_in_clean_air().ok();

    loop {
        if let Ok(ppm) = mq.read_gas_ppm(GasType::CO2) {
            // Do something with ppm (e.g., print over serial)
        }
    }
}

