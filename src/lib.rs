#![no_std]

use embedded_hal::adc::{OneShot, Channel};
use core::marker::PhantomData;
use libm::powf;

/// Represents the types of gases that the MQ135 sensor can detect.
#[derive(Copy, Clone, Debug)]
pub enum GasType {
    /// Carbon Dioxide (CO2)
    CO2,
    /// Ammonia (NH3)
    NH3,
    /// Benzene
    Benzene,
    /// Smoke (general smoke detection)
    Smoke,
}

/// Returns empirical constants (a, b) for the gas concentration formula
/// for each gas type. These constants come from datasheet curves.
///
/// The formula used is: ppm = a * (Rs/R0)^b
fn gas_constants(gas: GasType) -> (f32, f32) {
    match gas {
        GasType::CO2 => (110.47, -2.862),
        GasType::NH3 => (102.2, -2.473),
        GasType::Benzene => (44.947, -3.445),
        GasType::Smoke => (26.572, -2.265),
    }
}

/// Driver for the MQ135 gas sensor.
///
/// - `ADC`: The ADC peripheral implementing the embedded-hal `OneShot` trait.
/// - `PIN`: The analog input pin used to read sensor voltage.
/// - `Word`: The ADC output word type (usually u16 or u12).
/// - `Error`: The error type returned by the ADC.
pub struct Mq135<ADC, PIN, Word, Error> {
    adc: ADC,
    pin: PIN,
    /// Sensor baseline resistance in clean air (R0).
    r0: f32,
    /// Load resistance (RL) connected to the sensor.
    rl: f32,
    /// Phantom type for ADC output word.
    _word: PhantomData<Word>,
    /// Phantom type for ADC error type.
    _error: PhantomData<Error>,
}

impl<ADC, PIN, Word, Error> Mq135<ADC, PIN, Word, Error>
where
    ADC: OneShot<ADC, Word, PIN, Error = Error>,
    Word: Into<u32>,
    PIN: Channel<ADC>,
{
    /// Creates a new MQ135 sensor driver instance.
    ///
    /// # Arguments
    /// * `adc` - The ADC peripheral instance.
    /// * `pin` - The analog input pin connected to MQ135 output.
    /// * `rl` - The load resistance value in kilo-ohms (usually 10kΩ).
    ///
    /// # Note
    /// The `r0` value is set to a default 10.0 and needs to be calibrated in clean air.
    pub fn new(adc: ADC, pin: PIN, rl: f32) -> Self {
        Mq135 {
            adc,
            pin,
            r0: 10.0, // Default value, must be calibrated
            rl,
            _word: PhantomData,
            _error: PhantomData,
        }
    }

    /// Calibrates the sensor in clean air environment.
    ///
    /// This method reads the current sensor resistance in clean air,
    /// computes the baseline resistance (R0), and stores it internally.
    ///
    /// # Returns
    /// * `Ok(())` on success.
    /// * `Err(Error)` if the ADC reading fails.
    pub fn calibrate_in_clean_air(&mut self) -> nb::Result<(), Error> {
        let adc_value: u32 = self.adc.read(&mut self.pin)?.into();
        let voltage = adc_value as f32 / 4095.0 * 3.3;
        let rs = self.rl * (3.3 - voltage) / voltage;
        self.r0 = rs / 3.6;
        Ok(())
    }

    /// Reads the gas concentration (in PPM) for a specific gas type.
    ///
    /// # Arguments
    /// * `gas` - The target gas type to measure.
    ///
    /// # Returns
    /// * `Ok(ppm)` where `ppm` is the parts-per-million value of the gas.
    /// * `Err(Error)` if the ADC reading fails.
    pub fn read_gas_ppm(&mut self, gas: GasType) -> nb::Result<f32, Error> {
        let adc_value: u32 = self.adc.read(&mut self.pin)?.into();

        let voltage = adc_value as f32 / 4095.0 * 3.3;
        let rs = self.rl * (3.3 - voltage) / voltage;
        let ratio = rs / self.r0;
        let (a, b) = gas_constants(gas);
        Ok(a * powf(ratio, b))
    }
}
