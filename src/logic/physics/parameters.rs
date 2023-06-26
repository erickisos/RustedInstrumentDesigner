use std::f64::consts::PI;

use crate::structs::parameters::PhysicalParameters;

// Wave impedance of a bore of nominal radius, in kg/(m^4.s)
pub fn wave_impedance(parameters: PhysicalParameters, radius: f64) -> f64 {
    return parameters.air_density * parameters.sound_speed / (PI * radius * radius);
}

pub fn get_epsilon_from_f(parameters: PhysicalParameters, frequency: f64, radius: f64) -> f64 {
    return parameters.epsilon_constant / (radius * frequency.sqrt());
}

pub fn frequency(parameters: PhysicalParameters, wave_number: f64) -> f64 {
    return wave_number / parameters.wave_number;
}

pub fn wave_number(parameters: PhysicalParameters, frequency: f64) -> f64 {
    return frequency * parameters.wave_number;
}

#[cfg(test)]
mod parameters_tests;
