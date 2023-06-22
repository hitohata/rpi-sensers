//! This is a wrapper of the thermo sensor and humidity sensor.
use dht11::{ DHT11, MeasurementResult, DHT11Error };
use rppal::hal::Delay;

pub struct ThermoHumiditySensor {
    sensor: DHT11,
    delay: Delay
}

impl ThermoHumiditySensor {
    pub fn new(pin: u8) -> Result<Self, DHT11Error> {
        let sensor = DHT11::new(pin).unwrap();
        let delay = Delay::new();

        Ok(ThermoHumiditySensor {
            sensor,
            delay
        })
    }

    pub fn get_data(&mut self) -> Result<MeasurementResult, DHT11Error> {
        self.sensor.read(&mut self.delay)
    }
}
