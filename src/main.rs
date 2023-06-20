mod error_display;

use dht11::DHT11;
use rppal::hal::Delay;
use tokio::sync::mpsc;
use led_light::LEDPin;
use std::sync::mpsc::{ Receiver };
use error_display::error_light_sync;

#[tokio::main]
async fn main() {

    let temperature_sensor_pin = 5;
    let temp_sensor_led = 6;

    let mut dht11 = DHT11::new(temperature_sensor_pin).unwrap();
    let mut dht11_led = LEDPin::new(temp_sensor_led).unwrap();

    let (tx, mut rx) = std::sync::mpsc::channel();

    let handle = std::thread::spawn(move || {
        let mut delay = Delay::new();

        let mut errored = false;

        loop {
            dht11_led.turn_on();
            match dht11.read(&mut delay) {
                Ok(temperature) => { 
                    println!("{:?}", temperature);
                    if errored {
                        tx.send(true).unwrap();
                    }
                },
                Err(e) => {
                    println!("{:?}", e);
                    tx.send(false).unwrap();
                    errored = true;
                }
            }
            dht11_led.turn_off();
            std::thread::sleep(std::time::Duration::from_secs(3));
        }

    });

    let error_light = std::thread::spawn(move || {
        error_light_sync(rx);
    });

    handle.join().unwrap();
    error_light.join().unwrap();
}
