use crate::material::Material;
use crate::math::vec3::Color;

pub struct DiffuseLight {
    pub emission_color: Color,
    pub intensity: f64
}

impl DiffuseLight {
    pub fn new(emission_color: Color, intensity: f64) -> Self {
        Self {
            emission_color,
            intensity
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self) -> Color {
        self.emission_color * self.intensity
    }
}