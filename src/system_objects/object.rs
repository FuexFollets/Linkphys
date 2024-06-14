use crate::system_objects::system_object_info::*;
use crate::system_objects::vector2d::Vector2D;
use uom::si;

#[derive(Copy, Clone)]
pub struct Object {
    pub object_number: usize,
    pub mass: Option<si::f64::Mass>,

    pub translational_motion: TranslationalMotion,
    pub rotational_motion: RotationalMotion,
}

impl Object {
    pub fn new(
        object_number: usize,
        mass: Option<si::f64::Mass>,
        translational_motion: TranslationalMotion,
        rotational_motion: RotationalMotion,
    ) -> Self {
        Self {
            object_number,
            mass,
            translational_motion,
            rotational_motion,
        }
    }

    pub fn position_info(&self) -> (Vector2D<si::f64::Length>, Angle) {
        (
            match self.translational_motion {
                TranslationalMotion::Frozen(motion) => motion.position,
                TranslationalMotion::Mobile(motion) => motion.position,
            },
            match self.rotational_motion {
                RotationalMotion::Frozen(motion) => motion.angular_position,
                RotationalMotion::Mobile(motion) => motion.angular_position,
            },
        )
    }
}
