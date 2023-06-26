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

// impl PhysicalParameters {
//     pub(crate) fn new(basic_parameters: BasicParameters) -> PhysicalParameters {
//         let pressure = basic_parameters.pressure.unwrap_or(101.325);
//         let humidity_saturation = basic_parameters.humidity_saturation.unwrap_or(45.0);
//         let temperature = normalize_temperature(
//             basic_parameters.temperature.unwrap_or(72.0),
//             basic_parameters
//                 .temperature_type
//                 .unwrap_or(TemperatureType::F),
//         );
//         let molar_co2 = basic_parameters.molar_co2.unwrap_or(0.00039);
//         let molar_water_vapour =
//             calculate_molar_water_vapour(humidity_saturation, pressure, temperature);
//         let molar_mass_dry_air = molar_co2_to_mass_dry_air(molar_co2);
//         let molar_mass_moist_air = calculate_mass_moist_air(molar_water_vapour, molar_mass_dry_air);
//         let mass_fraction_water_vapour =
//             calculate_mass_water_vapour(molar_water_vapour, molar_mass_moist_air);
//         let mass_fraction_co2 = calculate_mass_fraction_co2(molar_co2, molar_mass_moist_air);
//         let humid_air_constant = calculate_humid_air_constant(molar_mass_moist_air);
//         let air_density = calculate_air_density(
//             pressure,
//             temperature,
//             humid_air_constant,
//             molar_water_vapour,
//         );
//         let humidity_ratio = molar_water_vapour_to_humidity_ratio(molar_water_vapour);

//         let air_dynamic_viscosity = temperature_to_air_dynamic_viscosity(temperature);
//         let water_vapour_dynamic_viscosity =
//             temperature_to_water_vapour_dynamic_viscosity(temperature);
//         // Dynamic viscosity of humid air, in kg/(m.s) or Pa.s.
//         let dynamic_viscosity_ratio =
//             air_vapour_viscosity_ratio(air_dynamic_viscosity, water_vapour_dynamic_viscosity);
//         let phi_air_vapour = calculate_phi_air_vapour(dynamic_viscosity_ratio, molar_mass_dry_air);
//         let phi_vapour_air = calculate_phi_vapour_air(dynamic_viscosity_ratio, molar_mass_dry_air);
//         let dynamic_viscosity = calculate_dynamic_viscosity(
//             air_dynamic_viscosity,
//             water_vapour_dynamic_viscosity,
//             humidity_ratio,
//             phi_air_vapour,
//             phi_vapour_air,
//         );
//         let specific_heat =
//             calculate_specific_heat(temperature, mass_fraction_water_vapour, mass_fraction_co2);
//         let specific_heats_ratio = specific_heat / (specific_heat - humid_air_constant);
//         let thermal_conductivity = calculate_thermal_conductivity(
//             temperature,
//             humidity_ratio,
//             phi_air_vapour,
//             phi_vapour_air,
//         );
//         let prandtl_number = dynamic_viscosity * specific_heat / thermal_conductivity;
//         let sound_speed = (specific_heats_ratio * humid_air_constant * temperature).sqrt();
//         let epsilon_constant = calculate_epsilon_constant(
//             air_density,
//             dynamic_viscosity,
//             specific_heats_ratio,
//             prandtl_number,
//         );
//         let alpha_constant = calculate_alpha_constant(
//             air_density,
//             dynamic_viscosity,
//             specific_heats_ratio,
//             sound_speed,
//             prandtl_number,
//         );
//         let wave_number = sound_speed_to_wave_number(sound_speed);
//         return PhysicalParameters {
//             temperature,
//             pressure,
//             humidity_saturation,
//             molar_co2,
//             molar_water_vapour,
//             air_density,
//             dynamic_viscosity,
//             specific_heat,
//             specific_heats_ratio,
//             thermal_conductivity,
//             prandtl_number,
//             sound_speed,
//             epsilon_constant,
//             alpha_constant,
//             wave_number,
//         };
//     }
// }

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
