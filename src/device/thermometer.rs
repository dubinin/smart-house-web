use rand::Rng;

use super::{Device, DisplayableDevice};

pub enum TemperatureScale {
    Kelvin,
    Celsius,
    Fahrenheit,
}

impl TemperatureScale {
    fn transform(&self, value: f32) -> f32 {
        match *self {
            Self::Celsius => value,
            Self::Fahrenheit => (value * (9.0 / 5.0)) + 32.0,
            Self::Kelvin => value + 273.15,
        }
    }

    fn from_env() -> TemperatureScale {
        std::env::var("TEMPERATURE_SCALE")
            .unwrap_or("C".to_string())
            .as_str()
            .into()
    }
}

impl From<&str> for TemperatureScale {
    fn from(value: &str) -> Self {
        match value {
            "C" => Self::Celsius,
            "F" => Self::Fahrenheit,
            "K" => Self::Kelvin,
            _ => Self::Celsius,
        }
    }
}

impl std::fmt::Display for TemperatureScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match *self {
            Self::Celsius => "°C",
            Self::Fahrenheit => "°F",
            Self::Kelvin => "K",
        };
        write!(f, "{description}")
    }
}

pub struct SmartThermometer {
    is_on: bool,
}

impl SmartThermometer {
    pub fn temperature(&self, scale: &TemperatureScale) -> f32 {
        let value = rand::thread_rng().gen_range(-25.0..35.00);
        scale.transform(value)
    }
}

impl Device for SmartThermometer {
    fn power(&self) -> u16 {
        20
    }

    fn is_on(&self) -> bool {
        self.is_on
    }

    fn is_socket(&self) -> bool {
        false
    }

    fn is_plugable(&self) -> bool {
        true
    }

    fn switch(&mut self) {
        self.is_on = !self.is_on;
    }
}

impl std::fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_on() {
            let scale = TemperatureScale::from_env();
            write!(
                f,
                "Умный термометр, температура: {} {}, потребляемая мощность: {}",
                self.temperature(&scale),
                scale,
                self.power()
            )
        } else {
            write!(f, "Умный термометр выключен.")
        }
    }
}

impl DisplayableDevice for SmartThermometer {}
