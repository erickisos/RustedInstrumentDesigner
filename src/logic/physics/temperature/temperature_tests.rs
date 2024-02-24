#[cfg(test)]
mod temperature_tests {
    use super::super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_can_normalize_any_unit() {
        let expected = 293.15;
        let actual = normalize_temperature(20., TemperatureType::C);
        assert_eq!(expected, actual);

        let actual = normalize_temperature(68., TemperatureType::F);
        assert_eq!(expected, actual);

        let actual = normalize_temperature(293.15, TemperatureType::K);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_can_handle_unexpected_values() {
        let actual = normalize_temperature(-300., TemperatureType::C);
        assert_eq!(0., actual);

        let actual = normalize_temperature(f64::NAN, TemperatureType::C);
        assert!(f64::is_nan(actual));
    }
}
