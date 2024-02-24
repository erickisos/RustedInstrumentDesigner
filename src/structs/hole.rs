use super::{parameters::PhysicalParameters, Builder};

static DEFAULT_HOLE_SIZE_MULT: f64 = 1.0;

#[derive(Debug)]
pub struct Hole {
    name: String,
    height: f64,
    position: f64,
    diameter: f64,
    inner_curvature_radius: f64,
}

#[derive(Default, Clone)]
pub struct HoleBuilder {
    name: Option<String>,
    height: Option<f64>,
    position: Option<f64>,
    diameter: Option<f64>,
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
        let _hole_size_mult = DEFAULT_HOLE_SIZE_MULT;
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
