use super::temperature::{normalize_temperature, TemperatureType};

struct BasicParameters {
    temperature: Option<f64>,                  // Temperature
    temperature_type: Option<TemperatureType>, // Temperature type
    pressure: Option<f64>,                     // Air Pressure in kPa
    molar_co2: Option<f64>,                    // Molar fraction of CO2 in mol/mol
    molar_water_vapour: Option<f64>,           // Molar fraction of water vapour in mol/mol
    humidity_saturation: Option<f64>,          // % of saturation
}

#[derive(Debug)]
pub(crate) struct CalculatedParameters {
    // Basic properties
    temperature: f64,         // Temperature in Kelvin degrees
    pressure: f64,            // Air Pressure in kPa
    molar_co2: f64,           // Molar fraction of CO2 in mol/mol
    molar_water_vapour: f64,  // Molar fraction of water vapour in mol/mol
    humidity_saturation: f64, // % of saturation
    // Calculated properties
    rho: f64,            // Air density kg/m^3
    eta: f64,            // Dynamic viscosity kg/(m.s)
    specific_heat: f64,  // isobaric_specific_heat J/(kg.K)
    gamma: f64,          // Specific heats ratio cp/cv (dimensionless)
    kappa: f64,          // Thermal conductivity in W/(m.K)
    prandtl_number: f64, // dimensionless
    sound_speed: f64,    // c in m/s
}

impl CalculatedParameters {
    fn new(basic_parameters: BasicParameters) -> CalculatedParameters {
        let pressure = basic_parameters.pressure.unwrap_or(101.325);
        let humidity_saturation = basic_parameters.humidity_saturation.unwrap_or(45.0);
        let temperature = normalize_temperature(
            basic_parameters.temperature.unwrap_or(72.0),
            basic_parameters
                .temperature_type
                .unwrap_or(TemperatureType::F),
        );
        let molar_water_vapour =
            calculate_molar_water_vapour(humidity_saturation, pressure, temperature);
        return CalculatedParameters {
            temperature,
            pressure,
            humidity_saturation,
            molar_co2: basic_parameters.molar_co2.unwrap_or(0.00039),
            molar_water_vapour,
            rho: calculate_air_density(pressure, temperature, molar_water_vapour),
            eta: todo!(),
            specific_heat: todo!(),
            gamma: todo!(),
            kappa: todo!(),
            prandtl_number: todo!(),
            sound_speed: todo!(),
        };
    }
}

fn calculate_vapour_pressure(temperature: f64) -> f64 {
    return 0.001
        * (1.2378847e-5 * temperature.powi(2) - 1.9121316e-2 * temperature + 33.93711047
            - (6.3431645e3 / temperature))
            .exp();
}

fn calculate_enhancement_factor(pressure: f64, temperature: f64) -> f64 {
    return 1.00062 + 3.14e-5 * pressure + 5.6e-7 * temperature.powi(2);
}

fn calculate_molar_water_vapour(humidity_saturation: f64, pressure: f64, temperature: f64) -> f64 {
    // Enhancement factor, from CIPM 2007.
    let enhancement_factor = calculate_enhancement_factor(pressure, temperature);
    // Saturated vapour pressure in kPa from CIPM-2007
    let saturated_vapour_pressure = calculate_vapour_pressure(temperature);
    return 0.01
        * humidity_saturation
        * enhancement_factor
        * (saturated_vapour_pressure / pressure);
}

fn calculate_air_density(pressure: f64, temperature: f64, molar_water_vapour: f64) -> f64 {
    // TODO: Check this function
    let humid_air_constant = 0.;
    let compressibility = 0.;
    return pressure * 1e3 / (compressibility * humid_air_constant * temperature);
}
