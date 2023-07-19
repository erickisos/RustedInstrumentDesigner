#[cfg(test)]

mod parameters_tests {
    use super::super::*;
    use crate::{
        logic::physics::temperature::TemperatureType,
        structs::{parameters::ParametersBuilder, Builder},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn it_can_build_from_temperature() {
        let parameters = ParametersBuilder::new()
            .with_temperature(20.0, TemperatureType::C)
            .build();
        assert_eq!(parameters.temperature, 293.15);
    }

    #[test]
    fn it_can_calculate_wave_impedance() {
        let parameters = ParametersBuilder::new()
            .with_temperature(20.0, TemperatureType::C)
            .build();
        let actual = wave_impedance(parameters, 0.006);
        assert_eq!(3681860.1456449446, actual);
    }

    #[test]
    fn it_can_build_at_sea_level() {
        let parameters = ParametersBuilder::new()
            .with_temperature(0.0, TemperatureType::C)
            .with_pressure(101.325)
            .with_humidity_saturation(0.0)
            .with_molar_co2(0.00039)
            .build();

        assert_eq!(331.43495349518037, parameters.sound_speed);
        assert_eq!(1.2931565359886497, parameters.air_density);
        assert_eq!(
            1.5171852624336475,
            get_epsilon_from_f(parameters, 1.0, 0.001)
        );
    }
    #[test]
    fn it_can_build_at_sea_level_normal_temp() {
        let parameters = ParametersBuilder::new()
            .with_temperature(20.0, TemperatureType::C)
            .with_pressure(101.325)
            .with_humidity_saturation(0.0)
            .with_molar_co2(0.00039)
            .build();
        assert_eq!(343.2878525006776, parameters.sound_speed);
        assert_eq!(1.2051536496637991, parameters.air_density);
        assert_eq!(
            1.6161041209381368,
            get_epsilon_from_f(parameters, 1.0, 0.001)
        );
    }

    #[test]
    fn it_can_build_at_saturated_air() {
        // Saturated air.
        let parameters = ParametersBuilder::new()
            .with_temperature(20.0, TemperatureType::C)
            .with_pressure(101.325)
            .with_humidity_saturation(100.0)
            .with_molar_co2(0.00039)
            .build();
        assert_eq!(348.3266788996193, parameters.sound_speed);
        assert_eq!(1.2050288905087325, parameters.air_density);
        assert_eq!(
            1.6208019965566673,
            get_epsilon_from_f(parameters, 1.0, 0.001)
        );
    }

    #[test]
    fn it_works_at_saturated_exhaled_air() {
        // Saturated, exhaled air.
        let parameters = ParametersBuilder::new()
            .with_temperature(37.0, TemperatureType::C)
            .with_pressure(101.325)
            .with_humidity_saturation(100.0)
            .with_molar_co2(0.04)
            .build();
        assert_eq!(363.99110139436897, parameters.sound_speed);
        assert_eq!(1.1571049683032653, parameters.air_density);
        assert_eq!(
            1.695452850871162,
            get_epsilon_from_f(parameters, 1.0, 0.001)
        );
    }

    #[test]
    fn it_works_at_1_km() {
        // At 1 km.
        // assert_eq!(89.996, PhysicalParameters.pressureAt(1000), 0.001);
        let parameters = ParametersBuilder::new()
            .with_temperature(20.0, TemperatureType::C)
            .with_pressure(89.996)
            .with_humidity_saturation(100.0)
            .with_molar_co2(0.00039)
            .build();
        assert_eq!(348.9733294703888, parameters.sound_speed);
        assert_eq!(1.0701809286452089, parameters.air_density);
        assert_eq!(
            1.7205031950036578,
            get_epsilon_from_f(parameters, 1.0, 0.001)
        );
    }
}
