pub(crate) enum TemperatureType {
    F,
    C,
    K,
}

const ALPHA_CONSTANT: f64 = 0.0;
const EPSILON_CONSTANT: f64 = 0.0;

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

fn epsilon(wave_number: &f64, radius: &f64) -> f64 {
    return ALPHA_CONSTANT / (radius * wave_number.sqrt());
}

fn epsilon_from_freq(frequency: &f64, radius: &f64) -> f64 {
    return EPSILON_CONSTANT / (radius * frequency.sqrt());
}
// pub(crate) struct Parameters {
//     // Temperature in Kelvin degrees
//     pub(crate) temperature: f64,
//     pressure: f64,
//     molar_co2: f64,
//     molar_water_vapour: f64,
//     humidity_saturation: f64,
// }

// impl Parameters {
//     pub(crate) fn new(temperature: f64, temperature_type: TemperatureType) -> Parameters {
//         return Parameters {
//             temperature: normalize_temperature(&temperature, temperature_type),
//             pressure: todo!(),
//             molar_co2: todo!(),
//             molar_water_vapour: todo!(),
//             humidity_saturation: todo!(),
//         };
//     }
// }
