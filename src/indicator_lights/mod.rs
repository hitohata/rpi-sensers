mod information_lights;
mod led_light;
pub use information_lights::{ sensor_light_initialize, InformationKind };

#[derive(Debug)]
pub enum LED {
    ON = 1,
    OFF = 0
}
