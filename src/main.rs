use dht11::DHT11;
use rppal::hal::Delay;

fn main() {

    let temperature_sensor_pin = 5;
    let temp_sensor_led = 6;

    let mut dht11 = DHT11::new(temperature_sensor_pin).unwrap();

    let handle = std::thread::spawn(move || {
        let mut delay = Delay::new();

        loop {
            println!("{:?}", dht11.read(&mut delay));
            std::thread::sleep(std::time::Duration::from_secs(10));
        }

    });

    handle.join().unwrap();
}
