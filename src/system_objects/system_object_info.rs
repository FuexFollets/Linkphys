use crate::system_objects::vector2d::Vector2D;
use crate::system_objects::{object::Object, particle::Particle};
use std::rc::Rc;
use uom::si::f64::{
    Acceleration, Angle, AngularAcceleration, AngularVelocity, Length, Mass, MomentOfInertia,
    Velocity,
};

#[derive(Copy, Clone)]
pub struct MobileTranslationalMotion {
    pub position: Vector2D<Length>,
    pub velocity: Vector2D<Velocity>,
    pub acceleration: Vector2D<Acceleration>,
}

impl Clone for MobileTranslationalMotion {
    fn clone(&self) -> Self {
        MobileTranslationalMotion {
            position: self.position.clone(),
            velocity: self.velocity.clone(),
            acceleration: self.acceleration.clone(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum TranslationalMotion {
    Frozen(MobileTranslationalMotion),
    Mobile(MobileTranslationalMotion),
}

impl Clone for TranslationalMotion {
    fn clone(&self) -> Self {
        match self {
            TranslationalMotion::Frozen(motion) => TranslationalMotion::Frozen(*motion),
            TranslationalMotion::Mobile(motion) => TranslationalMotion::Mobile(*motion),
        }
    }
}

#[derive(Copy, Clone)]
pub struct MobileRotationalMotion {
    pub angular_position: Angle,
    pub angular_velocity: AngularVelocity,
    pub angular_acceleration: AngularAcceleration,
}

impl Clone for MobileRotationalMotion {
    fn clone(&self) -> Self {
        MobileRotationalMotion {
            angular_position: self.angular_position.clone(),
            angular_velocity: self.angular_velocity.clone(),
            angular_acceleration: self.angular_acceleration.clone(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum RotationalMotion {
    Frozen(MobileRotationalMotion),
    Mobile(MobileRotationalMotion),
}

impl Clone for RotationalMotion {
    fn clone(&self) -> Self {
        match self {
            RotationalMotion::Frozen(motion) => RotationalMotion::Frozen(*motion),
            RotationalMotion::Mobile(motion) => RotationalMotion::Mobile(*motion),
        }
    }
}

#[derive(Copy, Clone)]
pub struct AngularPosition {
    pub angle: Angle,
    pub radius: Length,
}

impl Clone for AngularPosition {
    fn clone(&self) -> Self {
        AngularPosition {
            angle: self.angle.clone(),
            radius: self.radius.clone(),
        }
    }
}

impl AngularPosition {
    pub fn new(angle: Angle, radius: Length) -> AngularPosition {
        AngularPosition { angle, radius }
    }

    pub fn get_position(&self) -> Vector2D<Length> {
        Vector2D::new(
            self.radius * self.angle.sin(),
            self.radius * self.angle.cos(),
        )
    }
}

impl Vector2D<Length> {
    pub fn angular_position(&self) -> AngularPosition {
        AngularPosition::new(
            self.y.atan2(self.x),
            (self.x * self.x + self.y * self.y).sqrt(),
        )
    }
}

#[derive(Copy, Clone)]
pub struct ObjectAttachment {
    pub particle: Rc<Particle>,
    pub object: Rc<Object>,
    pub attachment_point_relative_to_object_center_of_mass: AngularPosition,
}

impl ObjectAttachment {
    pub fn new(
        particle: Rc<Particle>,
        object: Rc<Object>,
        attachment_point_relative_to_object_center_of_mass: AngularPosition,
    ) -> ObjectAttachment {
        ObjectAttachment {
            particle,
            object,
            attachment_point_relative_to_object_center_of_mass,
        }
    }

    pub fn get_actual_attachment_point_position(&self) -> Vector2D<Length> {
        let (object_position, object_rotation) = self.object.position_info();

        let attachment_point_relative_to_space = AngularPosition::new(
            self.attachment_point_relative_to_object_center_of_mass
                .angle
                + object_rotation,
            self.attachment_point_relative_to_object_center_of_mass
                .radius,
        );

        object_position + attachment_point_relative_to_space.get_position()
    }
}
