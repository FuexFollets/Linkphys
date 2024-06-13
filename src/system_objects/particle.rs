use crate::system_objects::vector2d::Vector2D;
use uom::si;

pub struct Particle {
    position: Vector2D<si::f64::Length>,
    mass: si::f64::Mass,
    particle_number: usize,
}

impl Particle {
    pub fn new(
        position: Vector2D<si::f64::Length>,
        mass: si::f64::Mass,
        particle_number: usize,
    ) -> Self {
        Self {
            position,
            mass,
            particle_number,
        }
    }
}
