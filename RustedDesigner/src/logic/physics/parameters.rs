#[derive(Debug)]
pub(crate) enum TemperatureType {
    F,
    C,
    K,
}

fn fahrenheit_to_celsius(degrees: &f64) -> f64 {
    return (*degrees + 40.) * 5. / 9. - 40.;
}

fn normalize_temperature(temperature: &f64, temperature_type: TemperatureType) -> f64 {
    return match temperature_type {
        TemperatureType::F => fahrenheit_to_celsius(temperature),
        TemperatureType::C => *temperature + 273.15,
        TemperatureType::K => *temperature,
    };
}

struct BasicParameters {
    temperature: Option<f64>,         // Temperature in Kelvin degrees
    pressure: Option<f64>,            // Air Pressure in kPa
    molar_co2: Option<f64>,           // Molar fraction of CO2 in mol/mol
    molar_water_vapour: Option<f64>,  // Molar fraction of water vapour in mol/mol
    humidity_saturation: Option<f64>, // % of saturation
}

#[derive(Debug)]
pub(crate) struct Parameters {
    // Calculated properties
    air_density: f64,            // kg/m^3
    dynamic_viscosity: f64,      // kg/(m.s)
    isobaric_specific_heat: f64, // J/(kg.K)
    ratio_specific_heat: f64,    // Specific heat ratio cp/cv (dimensionless)
    thermal_conductivity: f64,   // Thermal conductivity in W/(m.K)
    prandtl_number: f64,         // dimensionless
    sound_speed: f64,            // c in m/s
}

impl Parameters {
    fn new(
        temperature: f64,
        temperature_type: TemperatureType,
        pressure: f64,
        humidity_saturation: f64,
        molar_co2: f64,
    ) -> Parameters {
    }

    pub fn new(temperature: f64, temperature_type: TemperatureType) -> Parameters {
        return Parameters {
            temperature: normalize_temperature(&temperature, temperature_type),
            pressure: todo!(),
            molar_co2: todo!(),
            molar_water_vapour: todo!(),
            humidity_saturation: todo!(),
        };
    }

    pub fn new() -> Parameters {
        return Parameters::new(&72.0, TemperatureType::F);
    }
}
