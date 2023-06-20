use std::sync::mpsc::{ Receiver };
use led_light::LEDPin;

pub fn error_light_sync(rx: Receiver<bool>) {
    let mut error_light = LEDPin::new(13).unwrap();

    while let Ok(state) = rx.recv() {
        if state {
            error_light.turn_off();
        } else {
            error_light.turn_on();
        }
    }
    
}
