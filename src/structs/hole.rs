use super::{parameters::PhysicalParameters, Builder};

#[derive(Debug)]
pub struct Hole {
    name: String,
    position: f64,
    diameter: f64,
    height: f64,
    inner_curvature_radius: f64,
}

#[derive(Default, Clone, Copy)]
pub struct HoleBuilder {
    name: Option<String>,
    position: Option<f64>,
    diameter: Option<f64>,
    height: Option<f64>,
    inner_curvature_radius: Option<f64>,
}

pub trait HoleCalculator {
    fn calc_transfer_matrix(
        self,
        is_open: bool,
        wave_number: f64,
        parameters: PhysicalParameters,
    ) -> f64;
}

impl HoleBuilder {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        return self;
    }

    pub fn with_position(mut self, position: f64) -> Self {
        self.position = Some(position);
        return self;
    }

    pub fn with_diameter(mut self, diameter: f64) -> Self {
        self.diameter = Some(diameter);
        return self;
    }

    pub fn with_height(mut self, height: f64) -> Self {
        self.height = Some(height);
        return self;
    }

    pub fn with_inner_curvature_radius(mut self, inner_curvature_radius: f64) -> Self {
        self.inner_curvature_radius = Some(inner_curvature_radius);
        return self;
    }
}

impl Builder<Hole> for HoleBuilder {
    fn new() -> Self {
        let hole_size_mult = DEFAULT_HOLE_SIZE_MULT;
        todo!()
    }

    fn build(self) -> Hole {
        todo!()
    }
}

impl HoleCalculator for Hole {
    fn calc_transfer_matrix(
        self,
        is_open: bool,
        wave_number: f64,
        parameters: PhysicalParameters,
    ) -> f64 {
        todo!()
    }
}
