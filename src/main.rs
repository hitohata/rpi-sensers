mod indicator_lights;

use dht11::DHT11;
use rppal::hal::Delay;
use indicator_lights::information_light::{ sensor_light_initialize, SensorsKind, LEDState };

#[tokio::main]
async fn main() {

    let temperature_sensor_pin = 5;

    let info_indicator_tx = sensor_light_initialize(6).await;
    let info_indicator_tx1 = info_indicator_tx.clone();

    let handle = tokio::spawn(async move {

        let mut dht11 = DHT11::new(temperature_sensor_pin).unwrap();
        let mut delay = Delay::new();

        let mut errored = false;

        loop {
            info_indicator_tx1.send(SensorsKind::ThermoHumidity(LEDState::ON)).await;
            
            match dht11.read(&mut delay) {
                Ok(temperature) => { 
                    println!("{:?}", temperature);
                    // if errored {
                    //     tx.send(true).unwrap();
                    // }
                },
                Err(e) => {
                    println!("{:?}", e);
                    // tx.send(false).unwrap();
                    errored = true;
                }
            }
            info_indicator_tx1.send(SensorsKind::ThermoHumidity(LEDState::OFF)).await;
            std::thread::sleep(std::time::Duration::from_secs(3));
        }

    });

    // let error_light = std::thread::spawn(move || {
    //     error_light_sync(rx);
    // });

    handle.await;
    // error_light.join().unwrap();
}
