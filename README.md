# mq135

![Crates.io](https://img.shields.io/crates/v/mq135?style=flat-square)  
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)  
![Rust](https://img.shields.io/badge/rust-embedded--hal-orange?style=flat-square)

---

## 🚀 Overview

**mq135** is a minimal, `#![no_std]` Rust driver for the **MQ135 air quality sensor**.  
It’s designed for embedded systems using the [`embedded-hal`](https://crates.io/crates/embedded-hal) ADC traits.

The MQ135 sensor detects air quality by measuring gases like CO₂, NH₃, Benzene, and Smoke via analog voltage readings. This driver converts those readings into gas concentration (PPM).

---

## ✨ Features

- ✅ Calibration support in clean air  
- ✅ Measures PPM values for multiple gases (CO₂, NH₃, Benzene, Smoke)  
- ✅ Uses generic `embedded-hal` traits for wide MCU compatibility  
- ✅ No standard library (`#![no_std]`), perfect for microcontrollers  
- ✅ Easy-to-use API for quick integration  

---

## 📦 Usage

First, add the crate to your dependencies:

```toml
[dependencies]
mq135 = "0.1"  # replace with latest version
embedded-hal = "0.2"  # or your compatible version
libm = "0.2"  # for math in no_std environment
````

Example code snippet:

```rust
use mq135::{Mq135, GasType};

// `adc` must implement `embedded_hal::adc::OneShot`
// `pin` is the ADC input pin connected to MQ135 sensor output

let mut sensor = Mq135::new(adc, pin, 10_000.0); // RL load resistance in Ohms

// Calibrate the sensor in clean air before readings
sensor.calibrate_in_clean_air().unwrap();

// Get gas concentration in PPM
let co2_ppm = sensor.read_gas_ppm(GasType::CO2).unwrap();

println!("CO2 Concentration: {:.2} ppm", co2_ppm);
```

---

## 🧪 Calibration

Calibration is necessary to set the baseline sensor resistance (`R0`) in clean air. This ensures accurate gas concentration measurements.

---

## 🌬️ Supported Gases

| Gas     | Description          |
| ------- | -------------------- |
| CO₂     | Carbon Dioxide       |
| NH₃     | Ammonia              |
| Benzene | Aromatic Hydrocarbon |
| Smoke   | General Smoke Levels |

---

## ⚙️ Dependencies

* [`embedded-hal`](https://crates.io/crates/embedded-hal) — Hardware abstraction layer for embedded devices
* [`libm`](https://crates.io/crates/libm) — Math functions compatible with `no_std` environments

---

## 📄 License

This project is licensed under the **MIT License** © 2025 Prabesh Shrestha.

---

## 🤝 Contributions

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/prabesh-shrestha/mq135/issues).

---

Thank you for using **mq135**!
Happy coding and clean air monitoring! 🌿
