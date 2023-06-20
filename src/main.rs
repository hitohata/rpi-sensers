use dht11::DHT11;
use rppal::hal::Delay;
use led_light::LEDPin;

#[tokio::main]
async fn main() {

    let temperature_sensor_pin = 5;
    let temp_sensor_led = 6;

    let mut dht11 = DHT11::new(temperature_sensor_pin).unwrap();
    let mut dht11_led = LEDPin::new(temp_sensor_led).unwrap();

    let handle = std::thread::spawn(move || {
        let mut delay = Delay::new();

        loop {
            dht11_led.turn_on();
            println!("{:?}", dht11.read(&mut delay));
            dht11_led.turn_off();
            std::thread::sleep(std::time::Duration::from_secs(10));
        }

    });

    handle.join().unwrap();
}

fn error_light() {
    let 
}
