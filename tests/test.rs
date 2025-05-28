use mq135::{GasType, Mq135};
use embedded_hal::adc::{OneShot, Channel};
use core::convert::Infallible;

/// Dummy pin used for testing
#[derive(Default)]
struct MockPin;

/// Dummy ADC which returns a constant ADC value for testing
struct MockADC {
    value: u16,
}

// Implement the Channel trait so MockPin can be used with MockADC
impl Channel<MockADC> for MockPin {
    type ID = ();

    fn channel() -> Self::ID {
        ()
    }
}

// Implement the OneShot trait so MockADC can work with MockPin
impl OneShot<MockADC, u16, MockPin> for MockADC {
    type Error = Infallible;

    fn read(&mut self, _pin: &mut MockPin) -> nb::Result<u16, Self::Error> {
        Ok(self.value)
    }
}

#[test]
fn test_mq135_workflow() {
    let adc_value = 2048; // Simulated mid-range ADC value
    let adc = MockADC { value: adc_value };
    let pin = MockPin::default();

    let mut sensor = Mq135::new(adc, pin, 10.0);

    // Calibrate in clean air
    assert!(sensor.calibrate_in_clean_air().is_ok());

    // Read CO2 ppm
    let ppm_result = sensor.read_gas_ppm(GasType::CO2);
    assert!(ppm_result.is_ok());

    let ppm = ppm_result.unwrap();
    println!("Simulated CO2 PPM: {}", ppm);

    assert!(ppm > 0.0); // Basic sanity check
}
