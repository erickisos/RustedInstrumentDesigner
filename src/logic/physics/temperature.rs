#[derive(Debug)]
pub(crate) enum TemperatureType {
    F,
    C,
    K,
}

fn celsius_to_kelvin(degrees: f64) -> f64 {
    return degrees + 273.15;
}

fn fahrenheit_to_celsius(degrees: f64) -> f64 {
    return (degrees + 40.) * 5. / 9. - 40.;
}

pub(crate) fn normalize_temperature(temperature: f64, temperature_type: TemperatureType) -> f64 {
    return match temperature_type {
        TemperatureType::F => celsius_to_kelvin(fahrenheit_to_celsius(temperature)),
        TemperatureType::C => celsius_to_kelvin(temperature),
        TemperatureType::K => temperature,
    };
}

#[cfg(test)]
mod temperature_tests;
