use crate::logic::physics::{
    calcs::{
        calculate_air_density, calculate_alpha_constant, calculate_dynamic_viscosity,
        calculate_epsilon_constant, calculate_molar_water_vapour, calculate_prandtl_number,
        calculate_sound_speed, calculate_specific_heat, calculate_specific_heats_ratio,
        calculate_thermal_conductivity, sound_speed_to_wave_number,
    },
    temperature::{normalize_temperature, TemperatureType},
};

pub struct PhysicalParameters {
    // Basic properties
    pub temperature: f64,         // Temperature in Kelvin degrees
    pub pressure: f64,            // Air Pressure in kPa
    pub molar_co2: f64,           // Molar fraction of CO2 in mol/mol
    pub molar_water_vapour: f64,  // Molar fraction of water vapour in mol/mol
    pub humidity_saturation: f64, // % of saturation
    // Calculated properties
    pub air_density: f64,          // Air density kg/m^3
    pub dynamic_viscosity: f64,    // Dynamic viscosity kg/(m.s)
    pub specific_heat: f64,        // isobaric_specific_heat J/(kg.K)
    pub specific_heats_ratio: f64, // Specific heats ratio cp/cv (dimensionless, gamma)
    pub thermal_conductivity: f64, // Thermal conductivity in W/(m.K), kappa
    pub prandtl_number: f64,       // dimensionless
    pub sound_speed: f64,          // c in m/s
    pub epsilon_constant: f64,     // Multiplier for calculating adjustment to complex wave number
    pub alpha_constant: f64,       // Multiplier for calculating adjustment to complex wave number
    pub wave_number: f64,          // Wave number k at 1 Hz in rad/m
}

#[derive(Default, Clone, Copy)]
pub struct ParametersBuilder {
    temperature: Option<f64>,         // Temperature in Kelvin degrees
    pressure: Option<f64>,            // Air Pressure in kPa
    molar_co2: Option<f64>,           // Molar fraction of CO2 in mol/mol
    humidity_saturation: Option<f64>, // % of saturation
}

impl ParametersBuilder {
    pub fn new() -> Self {
        let temperature = normalize_temperature(72.0, TemperatureType::F);
        let humidity_saturation = 45.0;
        let molar_co2 = 0.00039;
        let pressure = 101.325;
        return Self {
            temperature: Some(temperature),
            pressure: Some(pressure),
            humidity_saturation: Some(humidity_saturation),
            molar_co2: Some(molar_co2),
            ..Default::default()
        };
    }

    pub fn with_pressure(mut self, pressure: f64) -> Self {
        self.pressure = Some(pressure);
        return self;
    }

    pub fn with_humidity_saturation(mut self, humidity_saturation: f64) -> Self {
        self.humidity_saturation = Some(humidity_saturation);
        return self;
    }

    pub fn with_molar_co2(mut self, molar_co2: f64) -> Self {
        self.molar_co2 = Some(molar_co2);
        return self;
    }

    pub fn with_temperature(mut self, temperature: f64, temperature_type: TemperatureType) -> Self {
        self.temperature = Some(normalize_temperature(temperature, temperature_type));
        return self;
    }

    pub fn build(self) -> PhysicalParameters {
        let temperature = self.temperature.unwrap();
        let pressure = self.pressure.unwrap();
        let humidity_saturation = self.humidity_saturation.unwrap();
        let molar_co2 = self.molar_co2.unwrap();

        return PhysicalParameters {
            pressure,
            molar_co2,
            temperature,
            humidity_saturation,
            molar_water_vapour: calculate_molar_water_vapour(
                pressure,
                temperature,
                humidity_saturation,
            ),
            air_density: calculate_air_density(
                pressure,
                temperature,
                humidity_saturation,
                molar_co2,
            ),
            dynamic_viscosity: calculate_dynamic_viscosity(
                pressure,
                temperature,
                humidity_saturation,
                molar_co2,
            ),
            specific_heat: calculate_specific_heat(
                pressure,
                temperature,
                humidity_saturation,
                molar_co2,
            ),
            specific_heats_ratio: calculate_specific_heats_ratio(
                pressure,
                temperature,
                humidity_saturation,
                molar_co2,
            ),
            thermal_conductivity: calculate_thermal_conductivity(
                pressure,
                temperature,
                humidity_saturation,
                molar_co2,
            ),
            prandtl_number: calculate_prandtl_number(
                pressure,
                temperature,
                humidity_saturation,
                molar_co2,
            ),
            sound_speed: calculate_sound_speed(
                pressure,
                temperature,
                humidity_saturation,
                molar_co2,
            ),
            epsilon_constant: calculate_epsilon_constant(
                pressure,
                temperature,
                humidity_saturation,
                molar_co2,
            ),
            alpha_constant: calculate_alpha_constant(
                pressure,
                temperature,
                humidity_saturation,
                molar_co2,
            ),
            wave_number: sound_speed_to_wave_number(
                pressure,
                temperature,
                humidity_saturation,
                molar_co2,
            ),
        };
    }
}
