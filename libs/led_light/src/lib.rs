use rppal::gpio::{ Gpio, OutputPin };
use thiserror::Error;

const BRINK_MIL_SEC: u64 = 200;

pub struct LEDPin {
    pin: OutputPin
}


impl LEDPin {
    pub fn new(pin_number: u8) -> Result<Self, LEDPinError> {
        let gpio = match Gpio::new() {
            Ok(gpio) => gpio,
            Err(_) => return Err(LEDPinError::LEDPinInitializationError("GPIO initialization failed.".into())),
        }; 

        let pin = match gpio.get(pin_number) {
            Ok(pin) => pin,
            Err(_) => return Err(LEDPinError::LEDPinInitializationError(format!("Pin, {}, initialization failed", pin_number)))
        };

        let mut out_pin = pin.into_output();
        out_pin.set_low();

        Ok(LEDPin {
            pin: out_pin
        })
    }

    /// turn on the LED.
    pub fn turn_on(&mut self) {
        self.pin.set_high();
    }

    /// turn off the LED.
    pub fn turn_off(&mut self) {
        self.pin.set_low();
    }

    pub fn brink(&mut self) {
        self.pin.set_high();
        std::thread::sleep(std::time::Duration::from_millis(BRINK_MIL_SEC));
        self.pin.set_low();
        std::thread::sleep(std::time::Duration::from_millis(BRINK_MIL_SEC))
    }

}

#[derive(Error, Debug)]
pub enum LEDPinError {
    #[error("LED Pin initialization failed. {0}")]
    LEDPinInitializationError(String)
}
