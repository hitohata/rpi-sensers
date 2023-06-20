use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use std::sync::Arc;
use led_light::LEDPin;

type SensorState = Arc<RwLock<State>>;

/// this struct holds an access state to sensors.
/// true means it is busy, and false means not being accessed.
#[derive(Default)]
struct State {
    thermo_humidity_sensor: bool,
}

/// this struct is just a wrapper of the State struct.
struct StateHandler {
    state: SensorState
}

impl StateHandler {
    async fn change_thermo_humidity_state(&self, state: LEDState) {
        let mut lock = self.state.write().await;
        lock.thermo_humidity_sensor = match state {
            LEDState::ON => true,
            LEDState::OFF => false
        };
    }   
}

#[derive(Debug)]
pub enum LEDState {
    ON = 1,
    OFF = 0
}

#[derive(Debug)]
pub enum SensorsKind {
    ThermoHumidity(LEDState)
}

/// this function handles sensor state.
/// to use the sensor indicator, this function must be called.
/// this function returns a sender of the channel, then the clients use it to send the indicator state.
pub async fn sensor_light_initialize(pin_number: u8) -> Sender<SensorsKind> {

    let (tx, mut rx) = tokio::sync::mpsc::channel::<SensorsKind>(10);

    let state = Arc::new(RwLock::new(State::default()));
    let sensor_handler = StateHandler { state: state.clone() };

    tokio::spawn(async move {
        let mut error_light = LEDPin::new(pin_number).unwrap();

        loop {
            let sensor_state = state.read().await;
            if sensor_state.thermo_humidity_sensor {
                error_light.turn_on();
            } else {
                error_light.turn_off();
            };
        }

    });

    tokio::spawn(async move {
        while let Some(command) = rx.recv().await {
            println!("{:?}", command);
            match command {
                SensorsKind::ThermoHumidity(state) => {
                    sensor_handler.change_thermo_humidity_state(state).await;
                }
            }
        }
    });

    tx
}
