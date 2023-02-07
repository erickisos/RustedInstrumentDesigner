enum TemperatureType {
    F,
    C,
    K,
}

pub(crate) struct Parameters {
    // Temperature in Kelvin degrees
    temperature: f64,
    pressure: f64,
    molar_co2: f64,
    molar_water_vapour: f64,
    humidity_saturation: f64,
}

impl Parameters {
    fn new(
        temperature: &f64,
        temperature_type: TemperatureType,
        pressure: &f64,
        relative_humidity: &f64,
        x_co2: &f64,
    ) -> Parameters {
        return Parameters {
            temperature: normalize_temperature(&temperature, temperature_type),
            pressure: todo!(),
            molar_co2: todo!(),
            molar_water_vapour: todo!(),
            humidity_saturation: todo!(),
        };
    }
}

fn fahrenheit_to_celsius(degrees: &f64) -> f64 {
    return (degrees + 40.) * 5. / 9. - 40.;
}

fn normalize_temperature(temperature: &f64, temperature_type: TemperatureType) -> f64 {
    return match temperature_type {
        TemperatureType::F => fahrenheit_to_celsius(temperature),
        TemperatureType::C => *temperature + 273.15,
        TemperatureType::K => *temperature,
    };
}

fn epsilon(wave_number: &f64, radius: &f64) {
    return ALPHA_CONSTANT / (radius * wave_number.sqrt());
}

fn epsilon_from_freq(frequency: &f64, radius: &f64) {
    return EPSILON_CONSTANT / (radius * frequency.sqrt());
}
