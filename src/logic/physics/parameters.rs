use super::temperature::{normalize_temperature, TemperatureType};
use super::{
    MOLAR_MASS_CO2, MOLAR_MASS_DRY_AIR, MOLAR_MASS_O2, MOLAR_MASS_WATER_VAPOUR,
    UNIVERSAL_GAS_CONSTANT,
};

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
        let molar_co2 = basic_parameters.molar_co2.unwrap_or(0.00039);
        let rho = calculate_air_density(pressure, temperature, molar_water_vapour, molar_co2);
        return CalculatedParameters {
            temperature,
            pressure,
            humidity_saturation,
            molar_co2,
            molar_water_vapour,
            rho,
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
    return 0.01 * humidity_saturation * enhancement_factor * saturated_vapour_pressure / pressure;
}

fn calculate_air_density(
    pressure: f64,
    temperature: f64,
    molar_water_vapour: f64,
    molar_fraction_co2: f64,
) -> f64 {
    // TODO: Prepare this function
    let molar_mass_dry_air =
        MOLAR_MASS_DRY_AIR + (MOLAR_MASS_CO2 - MOLAR_MASS_O2) * molar_fraction_co2;
    let molar_mass_moist_air =
        (1.0 - molar_water_vapour) * molar_mass_dry_air + molar_water_vapour * 0.;
    let humid_air_constant = UNIVERSAL_GAS_CONSTANT / (0.001 * molar_mass_moist_air);
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
fn calculate_dynamic_viscosity(
    temperature: f64,
    molar_mass_dry_air: f64,
    molar_water_vapour: f64,
) -> f64 {
    // TODO: Prepare this function
    // Dynamic viscosity of dry air, using Sutherland's formula, in kg/(m.s) or Pa.s.
    // from McQuillan, et al., 1984 (Reid, 1966).
    let air_dynamic_viscosity = 1.4592e-6 * temperature.powf(1.5) / (temperature + 109.1);

    // Dynamic viscosity of water vapour in air,
    // linear regression line from Tsilingiris, 2007, corrected for magnitude.
    let water_vapour_dynamic_viscosity = 8.058131868e-6 + temperature * 4.000549451e-8;

    // Dynamic viscosity of humid air, in kg/(m.s) or Pa.s.
    let dynamic_viscosity_ratio = (air_dynamic_viscosity / water_vapour_dynamic_viscosity).sqrt();
    let humidity_ratio = molar_water_vapour / (1.0 - molar_water_vapour);
    // double humidityRatio = m_xv/(1.0d-m_xv);
    // double phiAV = 0.5d*Math.pow(1.0d + etaRatio*Math.pow(Mv/Ma,0.25d),2.0d)
    // 				/Math.sqrt(2.0d*(1.0d+(Ma/Mv)));
    // double phiVA = 0.5d*Math.pow(1.0d + Math.pow(Ma/Mv,0.25d)/etaRatio,2.0d)
    // 				/Math.sqrt(2.0d*(1.0d+(Mv/Ma)));
    // mEta = etaAir/(1.0 + phiAV*humidityRatio)
    // 			+ humidityRatio*etaVapour/(humidityRatio + phiVA);
    let phi_air_vapour = 0.5
        * (1.0
            + dynamic_viscosity_ratio * (MOLAR_MASS_WATER_VAPOUR / molar_mass_dry_air).powf(0.25))
        .powf(2.)
        / (2.0 * (1.0 + (molar_mass_dry_air / MOLAR_MASS_WATER_VAPOUR))).sqrt();

    return air_dynamic_viscosity / (1.0 + phi_air_vapour * humidity_ratio);
}
