use tokio::sync::mpsc;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Default)]
struct State {
    thermo_sensor: bool,
    humidity_sensor: bool,
    error: bool
}

pub struct SensorsState {
    state: Arc<RwLock<State>>
}

impl SensorsState {
    fn new() -> Self {
        let state = Arc::new(RwLock::new(State::default()));
        SensorsState {
            state    
        }
    }

    async fn change_temperature_humidity_state(&self, state: bool) {
        
    }
}
