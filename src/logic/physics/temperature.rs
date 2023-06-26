pub enum TemperatureType {
    F,
    C,
    K,
}

pub fn kelvin_to_celsius(degrees: f64) -> f64 {
    return (degrees - 273.15).clamp(-273.15, f64::INFINITY);
}

pub fn celsius_to_kelvin(degrees: f64) -> f64 {
    return degrees.clamp(-273.15, f64::INFINITY) + 273.15;
}

pub fn fahrenheit_to_celsius(degrees: f64) -> f64 {
    return (degrees + 40.) * 5. / 9. - 40.;
}

pub(crate) fn normalize_temperature(temperature: f64, temperature_type: TemperatureType) -> f64 {
    return match temperature_type {
        TemperatureType::F => celsius_to_kelvin(fahrenheit_to_celsius(temperature)),
        TemperatureType::C => celsius_to_kelvin(temperature),
        TemperatureType::K => temperature.clamp(0., f64::INFINITY),
    };
}

#[cfg(test)]
mod temperature_tests;
