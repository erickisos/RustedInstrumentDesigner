#[cfg(test)]
mod temperature_tests {
    use super::super::*;

    #[test]
    fn it_transform_to_celsius() {
        let expected = 293.1;
        let actual = normalize_temperature(20., TemperatureType::C);
        assert_eq!(expected, actual);
    }
}
