pub struct Note {
    name: String,
    frequency: f64,
    frequency_min: f64,
    frequency_max: f64,
}

pub fn cents(f1: &f64, f2: &f64) -> f64 {
    return (f2 / f1).log2();
}
