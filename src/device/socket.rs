use std::collections::{hash_map::Values, HashMap};

use super::{DatabaseDevice, Device, DisplayableDevice};

/// Структура умной розетки, которая хранит ссылки на устройства подключенные к ней.
pub struct SmartSocket<'a> {
    is_on: bool,
    devices: HashMap<String, &'a dyn Device>,
}

pub struct SmartSocketIterator<'a> {
    iter: Values<'a, String, &'a dyn Device>,
}

impl<'a> Device for SmartSocket<'a> {
    /// Значение потребляемой мощности. Есть сумма потребляемых мощностей подключенных
    /// к розетки других устройств. У самой розетки потребляемая мощность равно 0.
    fn power(&self) -> u16 {
        self.into_iter().map(|device| device.power()).sum()
    }

    fn is_on(&self) -> bool {
        self.is_on
    }

    fn is_socket(&self) -> bool {
        true
    }

    fn is_plugable(&self) -> bool {
        false
    }

    fn switch(&mut self) {
        self.is_on = !self.is_on;
    }
}

/// Трейт Display используется для составления отчета по розетки.
impl<'a> std::fmt::Display for SmartSocket<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self.is_on {
            true => "включена",
            false => "выключена",
        };
        write!(
            f,
            "Умная розетка: {}, подключено устройств: {}, потребляемая мощность: {}",
            status,
            self.devices.len(),
            self.power()
        )
    }
}

impl<'a> DisplayableDevice for SmartSocket<'a> {}

/// Реализация итератора для разетки. Происходит обход по устройствам подключенным к розетке.
impl<'a> Iterator for SmartSocketIterator<'a> {
    type Item = &'a dyn Device;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.iter.next();
        match value {
            Some(v) => Some(*v),
            None => None,
        }
    }
}

/// Преобразование розетки в итератор.
impl<'a> IntoIterator for &'a SmartSocket<'a> {
    type Item = &'a dyn Device;
    type IntoIter = SmartSocketIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SmartSocketIterator {
            iter: self.devices.values(),
        }
    }
}

impl<'a> From<&DatabaseDevice> for SmartSocket<'a> {
    fn from(value: &DatabaseDevice) -> Self {
        Self {
            is_on: value.is_on,
            devices: HashMap::new(),
        }
    }
}
