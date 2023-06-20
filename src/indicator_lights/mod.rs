mod information_lights;
pub use information_lights::{ sensor_light_initialize, InformationKind };

#[derive(Debug)]
pub enum LED {
    ON = 1,
    OFF = 0
}
