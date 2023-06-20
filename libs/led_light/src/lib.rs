use rppal::gpio::{ Gpio, OutputPin };
use thiserror::Error;
use std::sync::{ Arc, RwLock };

const BLINK_MIL_SEC: u64 = 100;

pub struct LEDPin {
    pin: OutputPin,
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
            pin: out_pin,
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

    fn blink(&mut self) {
        self.pin.set_high();
        std::thread::sleep(std::time::Duration::from_millis(BLINK_MIL_SEC));
        self.pin.set_low();
        std::thread::sleep(std::time::Duration::from_millis(BLINK_MIL_SEC))
    }

}

pub struct BlinkingLEDPin {
    pin: u8,
    blinking: Arc<RwLock<bool>>
}

impl BlinkingLEDPin {
    pub fn new(pin_number: u8) -> Result<Self, LEDPinError> {

        let gpio = match Gpio::new() {
            Ok(gpio) => gpio,
            Err(_) => return Err(LEDPinError::LEDPinInitializationError("GPIO initialization failed.".into())),
        }; 

        let pin = match gpio.get(pin_number) {
            Ok(pin) => pin,
            Err(_) => return Err(LEDPinError::LEDPinInitializationError(format!("Pin, {}, initialization failed", pin_number)))
        };

        pin.into_output_low();

        Ok(BlinkingLEDPin {
            pin: pin_number,
            blinking: Arc::new(RwLock::new(false))
        })
    }

    pub fn start_blinking(&mut self) -> Result<(), LEDPinError> {

        *self.blinking.write().unwrap() = true;

        let is_blinking = self.blinking.clone();

        let gpio = match Gpio::new() {
            Ok(gpio) => gpio,
            Err(_) => return Err(LEDPinError::GPIOInitializationFailed(self.pin).into())
        };

        let pin = match gpio.get(self.pin) {
            Ok(pin) => pin,
            Err(_) => {
                return Err(LEDPinError::PinInitializationFailed(self.pin));
            }
        };

        std::thread::spawn(move || {
            let mut out_pin = pin.into_output_low();

            loop {

                if !*is_blinking.read().unwrap() {
                    break;
                };

                out_pin.set_high();
                std::thread::sleep(std::time::Duration::from_millis(BLINK_MIL_SEC));
                out_pin.set_low();
                std::thread::sleep(std::time::Duration::from_millis(BLINK_MIL_SEC));
            }
            
        });

        Ok(())
    }

    pub fn stop_blinking(&mut self) {
        *self.blinking.write().unwrap() = false;
    } 


}

#[derive(Error, Debug)]
pub enum LEDPinError {
    #[error("LED Pin initialization failed. {0}")]
    LEDPinInitializationError(String),
    #[error("GPIO initialization failed. PIN: {0}")]
    GPIOInitializationFailed(u8),
    #[error("Pin initialization failed. PIN: {0}")]
    PinInitializationFailed(u8)
}
