use std::sync::mpsc::Sender;
use std::sync::RwLock;
use std::sync::Arc;
use led_light::{ LEDPin, BlinkingLEDPin };
use crate::indicator_lights::LED;

type SensorStateType = Arc<RwLock<InformationState>>;
type ErrorStateType = Arc<RwLock<ErrorState>>;

/// this struct holds an access state to sensors.
/// true means it is busy, and false means not being accessed.
#[derive(Default)]
struct InformationState {
    thermo_humidity_sensor: bool,
    network_access: bool
}

#[derive(Default)]
struct ErrorState {
    sensor_error: bool
}

/// this struct is just a wrapper of the State struct.
struct StateHandler {
    state: SensorStateType,
    error: ErrorStateType
}

impl StateHandler {
    fn change_thermo_humidity_state(&self, state: LED) {
        let mut lock = self.state.write().unwrap();
        lock.thermo_humidity_sensor = match state {
            LED::ON => true,
            LED::OFF => false
        };
    }   

    fn change_network_state(&self, state: LED) {
        let mut lock = self.state.write().unwrap();
        lock.network_access = match state {
            LED::ON => true,
            LED::OFF => false
        }
    }

    fn change_error_state(&self, state: LED) {
        let mut lock = self.error.write().unwrap();
        lock.sensor_error = match state {
            LED::ON => true,
            LED::OFF => false
        }
    }
}


#[derive(Debug)]
pub enum InformationKind {
    ThermoHumidity(LED),
    NetworkAccess(LED),
    SensorError(LED)
}

/// this function handles sensor state.
/// to use the sensor indicator, this function must be called.
/// this function returns a sender of the channel, then the clients use it to send the indicator state.
pub fn sensor_light_initialize(sensor_pin_number: u8, network_pin_number: u8, error_pin_number: u8) -> Sender<InformationKind> {

    let (tx, rx) = std::sync::mpsc::channel::<InformationKind>();

    let information_state = Arc::new(RwLock::new(InformationState::default()));
    let error_state = Arc::new(RwLock::new(ErrorState::default()));
    let sensor_handler = StateHandler { state: information_state.clone(), error: error_state.clone() };

    std::thread::spawn(move || {
        let mut sensor_light = LEDPin::new(sensor_pin_number).unwrap();
        let mut network_light = BlinkingLEDPin::new(network_pin_number).unwrap();

        loop {
            let information_state_read = information_state.read().unwrap();
            match information_state_read.thermo_humidity_sensor {
                true => sensor_light.turn_on(),
                false => sensor_light.turn_off()
            };

            match information_state_read.network_access {
                true => {network_light.start_blinking();},
                false => {network_light.stop_blinking();}
            };
        }
    });

    std::thread::spawn(move || {

        let mut error_light = LEDPin::new(error_pin_number).unwrap();

        loop {
            let mut error_state_read = error_state.read().unwrap();

            match error_state_read.sensor_error {
                true => { error_light.turn_on(); },
                false => { error_light.turn_off(); }
            }

        }
    });

    std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                InformationKind::ThermoHumidity(state) => {
                    sensor_handler.change_thermo_humidity_state(state);
                },
                InformationKind::NetworkAccess(state) => {
                    sensor_handler.change_network_state(state);
                },
                InformationKind::SensorError(state) => {
                    sensor_handler.change_error_state(state);
                }
            }
        }
    });

    tx
}
