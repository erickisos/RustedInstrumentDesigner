use std::f64::consts::PI;

use crate::structs::parameters::PhysicalParameters;

// Wave impedance of a bore, nominal radius r.
pub(crate) fn calculate_wave_impedance(parameters: PhysicalParameters, radius: f64) -> f64 {
    return parameters.air_density * parameters.sound_speed / (PI * radius.powf(2.0));
}

pub fn get_epsilon_from_f(parameters: PhysicalParameters, frequency: f64, radius: f64) -> f64 {
    return parameters.epsilon_constant / (radius * frequency.sqrt());
}

#[cfg(test)]
mod parameters_tests;
