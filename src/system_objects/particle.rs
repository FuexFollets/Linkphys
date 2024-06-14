use crate::system_objects::system_object_info::*;
use crate::system_objects::vector2d::*;
use uom::si;

#[derive(Copy, Clone)]
pub struct Particle {
    pub particle_number: usize,
    pub mass: si::f64::Mass,

    pub translational_motion: TranslationalMotion,
}

impl Clone for Particle {
    fn clone(&self) -> Self {
        Particle {
            particle_number: self.particle_number,
            mass: self.mass,
            translational_motion: self.translational_motion,
        }
    }
}

impl Particle {
    pub fn new(
        particle_number: usize,
        position: Vector2D<si::f64::Length>,
        mass: si::f64::Mass,
        translational_motion: TranslationalMotion,
    ) -> Particle {
        Particle {
            particle_number,
            mass,
            translational_motion,
        }
    }
}

impl Particle {
    pub fn freeze(&mut self) {
        match self.translational_motion {
            TranslationalMotion::Mobile(motion) => {
                self.translational_motion = TranslationalMotion::Frozen(motion);
            }
            _ => {}
        }
    }

    pub fn unfreeze(&mut self) {
        match self.translational_motion {
            TranslationalMotion::Frozen(motion) => {
                self.translational_motion = TranslationalMotion::Mobile(motion);
            }
            _ => {}
        }
    }

    pub fn is_frozen(&self) -> bool {
        match self.translational_motion {
            TranslationalMotion::Frozen(_) => true,
            _ => false,
        }
    }

    pub fn toggle_freeze(&mut self) {
        if self.is_frozen() {
            self.unfreeze();
        } else {
            self.freeze();
        }
    }

    pub fn take_step(&mut self, delta_time: si::f64::Time) {
        match self.translational_motion {
            TranslationalMotion::Mobile(ref mut motion) => {
                motion.position += vector2d_mul!(motion.velocity, delta_time);
                motion.velocity += vector2d_mul!(motion.acceleration, delta_time);
            }
            _ => {}
        }
    }
}
