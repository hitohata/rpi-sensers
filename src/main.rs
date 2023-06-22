mod indicator_lights;
mod sensors;

use indicator_lights::{ sensor_light_initialize, InformationKind };
use indicator_lights::LED;
use sensors::temperature_humidity::ThermoHumiditySensor;

#[tokio::main]
async fn main() {

    let temperature_sensor_pin = 5;

    let info_indicator_tx = sensor_light_initialize(6, 19, 13);
    let info_indicator_tx1 = info_indicator_tx.clone();

    let handle = std::thread::spawn(move || {

        let mut temperature_sensor = match ThermoHumiditySensor::new(temperature_sensor_pin) {
            Ok(sensor) => sensor,
            Err(_) => {
                info_indicator_tx1.send(InformationKind::SensorError(LED::ON));
                return;
            }
        }; 

        let mut errored = false;

        loop {
            info_indicator_tx1.send(InformationKind::ThermoHumidity(LED::ON));
            
            match temperature_sensor.get_data() {
                Ok(temperature) => { 
                    println!("{:?}", temperature);
                    if errored {
                        info_indicator_tx1.send(InformationKind::SensorError(LED::OFF)).unwrap();
                    }
                },
                Err(e) => {
                    println!("{:?}", e);
                    info_indicator_tx1.send(InformationKind::SensorError(LED::ON)).unwrap();
                    errored = true;
                }
            }
            info_indicator_tx1.send(InformationKind::ThermoHumidity(LED::OFF));
            std::thread::sleep(std::time::Duration::from_secs(3));
        }

    });


    handle.join().unwrap();
}
