use std::f64::consts::PI;

use super::temperature::kelvin_to_celsius;
use super::{
    MOLAR_MASS_CO2, MOLAR_MASS_DRY_AIR, MOLAR_MASS_O2, MOLAR_MASS_WATER_VAPOUR,
    UNIVERSAL_GAS_CONSTANT,
};

pub fn calculate_mass_water_vapour(molar_water_vapour: f64, molar_mass_moist_air: f64) -> f64 {
    return molar_water_vapour * MOLAR_MASS_WATER_VAPOUR / molar_mass_moist_air;
}

pub fn molar_co2_to_mass_dry_air(molar_co2: f64) -> f64 {
    return MOLAR_MASS_DRY_AIR + (MOLAR_MASS_CO2 - MOLAR_MASS_O2) * molar_co2;
}

pub fn calculate_mass_moist_air(molar_water_vapour: f64, molar_mass_dry_air: f64) -> f64 {
    return (1.0 - molar_water_vapour) * molar_mass_dry_air + molar_water_vapour * 0.;
}

pub fn calculate_mass_fraction_co2(molar_co2: f64, molar_mass_moist_air: f64) -> f64 {
    return molar_co2 * MOLAR_MASS_CO2 / molar_mass_moist_air;
}

pub fn temperature_to_air_dynamic_viscosity(temperature: f64) -> f64 {
    // Dynamic viscosity of dry air, using Sutherland's formula, in kg/(m.s) or Pa.s.
    // from McQuillan, et al., 1984 (Reid, 1966).
    return 1.4592e-6 * temperature.powf(1.5) / (temperature + 109.1);
}

pub fn calculate_epsilon_constant(
    pressure: f64,
    temperature: f64,
    humidity_saturation: f64,
    molar_co2: f64,
) -> f64 {
    let specific_heats_ratio =
        calculate_specific_heats_ratio(pressure, temperature, humidity_saturation, molar_co2);
    let prandtl_number =
        calculate_prandtl_number(pressure, temperature, humidity_saturation, molar_co2);
    let dynamic_viscosity =
        calculate_dynamic_viscosity(pressure, temperature, humidity_saturation, molar_co2);
    let air_density = calculate_air_density(pressure, temperature, humidity_saturation, molar_co2);
    return 1.0 / (2.0 * PI.sqrt())
        * (dynamic_viscosity / air_density).sqrt()
        * (1.0 + (specific_heats_ratio - 1.0) / prandtl_number.sqrt());
}

pub fn calculate_alpha_constant(
    pressure: f64,
    temperature: f64,
    humidity_saturation: f64,
    molar_co2: f64,
) -> f64 {
    let sound_speed = calculate_sound_speed(pressure, temperature, humidity_saturation, molar_co2);
    let specific_heats_ratio =
        calculate_specific_heats_ratio(pressure, temperature, humidity_saturation, molar_co2);
    let prandtl_number =
        calculate_prandtl_number(pressure, temperature, humidity_saturation, molar_co2);
    let dynamic_viscosity =
        calculate_dynamic_viscosity(pressure, temperature, humidity_saturation, molar_co2);
    let air_density = calculate_air_density(pressure, temperature, humidity_saturation, molar_co2);
    return (dynamic_viscosity / (2.0 * air_density * sound_speed)).sqrt()
        * (1.0 + (specific_heats_ratio - 1.0) / prandtl_number.sqrt());
}

pub fn dynamic_viscosity_ratio(
    air_dynamic_viscosity: f64,
    water_vapour_dynamic_viscosity: f64,
) -> f64 {
    return (air_dynamic_viscosity / water_vapour_dynamic_viscosity).sqrt();
}

pub fn sound_speed_to_wave_number(
    pressure: f64,
    temperature: f64,
    humidity_saturation: f64,
    molar_co2: f64,
) -> f64 {
    let sound_speed = calculate_sound_speed(pressure, temperature, humidity_saturation, molar_co2);
    return 2.0 * PI / sound_speed;
}

pub fn temperature_to_water_vapour_dynamic_viscosity(temperature: f64) -> f64 {
    // Dynamic viscosity of water vapour in air,
    // linear regression line from Tsilingiris, 2007, corrected for magnitude.
    return 8.058131868e-6 + temperature * 4.000549451e-8;
}

pub fn temperature_to_vapour_pressure(temperature: f64) -> f64 {
    return 0.001
        * (1.2378847e-5 * temperature.powi(2) - 1.9121316e-2 * temperature + 33.93711047
            - (6.3431645e3 / temperature))
            .exp();
}

pub fn molar_water_vapour_to_humidity_ratio(molar_water_vapour: f64) -> f64 {
    return molar_water_vapour / (1.0 - molar_water_vapour);
}

pub fn calculate_humid_air_constant(molar_mass_moist_air: f64) -> f64 {
    return UNIVERSAL_GAS_CONSTANT / (0.001 * molar_mass_moist_air);
}

pub fn calculate_enhancement_factor(pressure: f64, temperature: f64) -> f64 {
    return 1.00062 + 3.14e-5 * pressure + 5.6e-7 * temperature.powi(2);
}

pub fn calculate_molar_water_vapour(
    pressure: f64,
    temperature: f64,
    humidity_saturation: f64,
) -> f64 {
    // Enhancement factor, from CIPM 2007.
    let enhancement_factor = calculate_enhancement_factor(pressure, temperature);
    // Saturated vapour pressure in kPa from CIPM-2007
    let saturated_vapour_pressure = temperature_to_vapour_pressure(temperature);
    return 0.01 * humidity_saturation * enhancement_factor * saturated_vapour_pressure / pressure;
}

pub fn calculate_air_density(
    pressure: f64,
    temperature: f64,
    humidity_saturation: f64,
    molar_co2: f64,
) -> f64 {
    let humid_air_constant = calculate_humid_air_constant(molar_co2_to_mass_dry_air(molar_co2));
    let molar_water_vapour =
        calculate_molar_water_vapour(pressure, temperature, humidity_saturation);
    let pascal_pressure = pressure * 1000.0;
    let compressibility = 1.0
        - pascal_pressure / temperature
            * (1.58123e-6 - 2.9331e-8 * temperature
                + 1.1043e-10 * temperature.powi(2)
                + (5.707e-6 - 2.051e-8 * temperature) * molar_water_vapour
                + (1.9898e-4 - 2.376e-6 * temperature) * molar_water_vapour.powi(2))
        + (pascal_pressure / temperature).powi(2)
            * (1.83e-11 - 0.765e-8 * molar_water_vapour.powi(2));
    return pressure * 1e3 / (compressibility * humid_air_constant * temperature);
}

// Dynamic viscosity,
pub fn calculate_dynamic_viscosity(
    pressure: f64,
    temperature: f64,
    humidity_saturation: f64,
    molar_co2: f64,
) -> f64 {
    let air_dynamic_viscosity = temperature_to_air_dynamic_viscosity(temperature);
    let water_vapour_dynamic_viscosity = temperature_to_water_vapour_dynamic_viscosity(temperature);
    let humidity_ratio = molar_water_vapour_to_humidity_ratio(calculate_molar_water_vapour(
        pressure,
        temperature,
        humidity_saturation,
    ));
    let phi_air_vapour = calculate_phi_air_vapour(
        dynamic_viscosity_ratio(
            temperature_to_air_dynamic_viscosity(temperature),
            temperature_to_water_vapour_dynamic_viscosity(temperature),
        ),
        molar_co2_to_mass_dry_air(molar_co2),
    );
    let phi_vapour_air = calculate_phi_vapour_air(
        dynamic_viscosity_ratio(
            temperature_to_air_dynamic_viscosity(temperature),
            temperature_to_water_vapour_dynamic_viscosity(temperature),
        ),
        molar_co2_to_mass_dry_air(molar_co2),
    );
    return air_dynamic_viscosity / (1.0 + phi_air_vapour * humidity_ratio)
        + humidity_ratio * water_vapour_dynamic_viscosity / (humidity_ratio + phi_vapour_air);
}

pub fn calculate_phi_air_vapour(dynamic_viscosity_ratio: f64, molar_mass_dry_air: f64) -> f64 {
    return 0.5
        * (1.0
            + dynamic_viscosity_ratio * (MOLAR_MASS_WATER_VAPOUR / molar_mass_dry_air).powf(0.25))
        .powf(2.)
        / (2.0 * (1.0 + (molar_mass_dry_air / MOLAR_MASS_WATER_VAPOUR))).sqrt();
}

pub fn calculate_phi_vapour_air(dynamic_viscosity_ratio: f64, molar_mass_dry_air: f64) -> f64 {
    return 0.5
        * (1.0
            + (molar_mass_dry_air / MOLAR_MASS_WATER_VAPOUR).powf(0.25) / dynamic_viscosity_ratio)
            .powf(2.)
        / (2.0 * (1.0 + (MOLAR_MASS_WATER_VAPOUR / molar_mass_dry_air))).sqrt();
}

pub fn calculate_specific_heat(
    pressure: f64,
    temperature: f64,
    humidity_saturation: f64,
    molar_co2: f64,
) -> f64 {
    // Isobaric specific heat, cp, in J/(kg.K).

    // Isobaric specific heat of air and water vapour, from Tsilingiris,
    // 2007,
    // with specific heat of air reduced by 2 J/kg.K to get gamma correct.
    let molar_water_vapour =
        calculate_molar_water_vapour(pressure, temperature, humidity_saturation);
    let molar_mass_dry_air = molar_co2_to_mass_dry_air(molar_co2);
    let molar_mass_moist_air = calculate_mass_moist_air(molar_water_vapour, molar_mass_dry_air);
    let mass_fraction_water_vapour =
        calculate_mass_water_vapour(molar_water_vapour, molar_mass_moist_air);
    let mass_fraction_co2 = calculate_mass_fraction_co2(molar_co2, molar_mass_moist_air);
    let celsius_temperature = kelvin_to_celsius(temperature);
    let air_specific_heat = 1032.0
        + temperature
            * (-0.284887
                + temperature
                    * (0.7816818e-3 + temperature * (-0.4970786e-6 + temperature * 0.1077024e-9)));
    let vapour_specific_heat =
        1869.10989 + celsius_temperature * (-0.2578421578 + celsius_temperature * 1.941058941e-2);
    // Isobaric specific heat of CO2, curve fit on available data.
    let co2_specific_heat = 817.02 + celsius_temperature * (1.0562 - celsius_temperature * 6.67e-4);
    return air_specific_heat * (1. - mass_fraction_water_vapour - mass_fraction_co2)
        + vapour_specific_heat * mass_fraction_water_vapour
        + co2_specific_heat * mass_fraction_co2;
}

pub fn calculate_specific_heats_ratio(
    pressure: f64,
    temperature: f64,
    humidity_saturation: f64,
    molar_co2: f64,
) -> f64 {
    let specific_heat =
        calculate_specific_heat(pressure, temperature, humidity_saturation, molar_co2);
    let humid_air_constant = calculate_humid_air_constant(calculate_mass_moist_air(
        calculate_molar_water_vapour(pressure, temperature, humidity_saturation),
        molar_co2_to_mass_dry_air(molar_co2),
    ));
    return specific_heat / (specific_heat - humid_air_constant);
}

// Thermal conductivity, in W/(m.K).
pub fn calculate_thermal_conductivity(
    pressure: f64,
    temperature: f64,
    humidity_saturation: f64,
    molar_co2: f64,
) -> f64 {
    // Thermal conductivity of dry air, using Sutherland's formula, from
    // McQuillan, et al., 1984.
    let humidity_ratio = molar_water_vapour_to_humidity_ratio(calculate_molar_water_vapour(
        pressure,
        temperature,
        humidity_saturation,
    ));
    let phi_air_vapour = calculate_phi_air_vapour(
        dynamic_viscosity_ratio(
            temperature_to_air_dynamic_viscosity(temperature),
            temperature_to_water_vapour_dynamic_viscosity(temperature),
        ),
        molar_co2_to_mass_dry_air(molar_co2),
    );
    let phi_vapour_air = calculate_phi_vapour_air(
        dynamic_viscosity_ratio(
            temperature_to_air_dynamic_viscosity(temperature),
            temperature_to_water_vapour_dynamic_viscosity(temperature),
        ),
        molar_co2_to_mass_dry_air(molar_co2),
    );
    let celsius_temperature = kelvin_to_celsius(temperature);
    let kappa_air = 2.3340e-3 * temperature.powf(1.5) / (temperature + 164.54);
    // Thermal conductivity of water vapour, from Tsirilingis, 2007.
    let kappa_vapour = 0.01761758242
        + celsius_temperature * (5.558941059e-5 + celsius_temperature * 1.663336663e-7);
    return kappa_air / (1.0 + phi_air_vapour * humidity_ratio)
        + humidity_ratio * kappa_vapour / (humidity_ratio + phi_vapour_air);
}

pub fn calculate_prandtl_number(
    pressure: f64,
    temperature: f64,
    humidity_saturation: f64,
    molar_co2: f64,
) -> f64 {
    let dynamic_viscosity =
        calculate_dynamic_viscosity(pressure, temperature, humidity_saturation, molar_co2);
    let specific_heat =
        calculate_specific_heat(pressure, temperature, humidity_saturation, molar_co2);
    let thermal_conductivity =
        calculate_thermal_conductivity(pressure, temperature, humidity_saturation, molar_co2);
    return dynamic_viscosity * specific_heat / thermal_conductivity;
}

pub fn calculate_sound_speed(
    pressure: f64,
    temperature: f64,
    humidity_saturation: f64,
    molar_co2: f64,
) -> f64 {
    let specific_heats_ratio =
        calculate_specific_heats_ratio(pressure, temperature, humidity_saturation, molar_co2);
    let humid_air_constant = calculate_humid_air_constant(calculate_mass_moist_air(
        calculate_molar_water_vapour(pressure, temperature, humidity_saturation),
        molar_co2_to_mass_dry_air(molar_co2),
    ));
    return (specific_heats_ratio * humid_air_constant * temperature).sqrt();
}
