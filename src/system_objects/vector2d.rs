use std::ops::{Add, Sub};

use uom::{self, Conversion};

/*
pub trait UOMQuantity<D, U, V>
where
    D: uom::si::Dimension + ?Sized,
    U: uom::si::Units<V> + ?Sized,
    V: uom::num_traits::Num + uom::Conversion<V>,
    <D as uom::si::Dimension>::Kind: uom::marker::Add,
    uom::si::Quantity<D, U, V>: Clone,
{
}
*/

#[macro_export]
macro_rules! vector2d_mul {
    ($lhs:expr, $rhs:expr) => {
        Vector2D {
            x: $lhs.x.clone() * $rhs.clone(),
            y: $lhs.y.clone() * $rhs.clone(),
        }
    };
}

#[macro_export]
macro_rules! vector2d_div {
    ($lhs:expr, $rhs:expr) => {
        Vector2D {
            x: $lhs.x.clone() / $rhs.clone(),
            y: $lhs.y.clone() / $rhs.clone(),
        }
    };
}

#[macro_export]
macro_rules! vector2d_dot {
    ($lhs:expr, $rhs:expr) => {
        $lhs.x.clone() * $rhs.x.clone() + $lhs.y.clone() * $rhs.y.clone()
    };
}

#[macro_export]
macro_rules! vector2d_cross {
    ($lhs:expr, $rhs:expr) => {
        $lhs.x.clone() * $rhs.y.clone() - $lhs.y.clone() * $rhs.x.clone()
    };
}

#[macro_export]
macro_rules! vector2d_hadamard {
    ($lhs:expr, $rhs:expr) => {
        Vector2D {
            x: $lhs.x.clone() * $rhs.x.clone(),
            y: $lhs.y.clone() * $rhs.y.clone(),
        }
    };
}

struct Vector2D<T: Clone> {
    x: T,
    y: T,
}

impl<T: Clone> Clone for Vector2D<T> {
    fn clone(&self) -> Self {
        Vector2D {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T: Clone> Add for Vector2D<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2D {
            x: self.x.clone() + rhs.x.clone(),
            y: self.y.clone() + rhs.y.clone(),
        }
    }
}

impl<T: Clone> Sub for Vector2D<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D {
            x: self.x.clone() - rhs.x.clone(),
            y: self.y.clone() - rhs.y.clone(),
        }
    }
}

trait GetValueF32<T> {
    fn value(&self) -> f32;
    fn new(value: f32) -> T;
}

trait GetValueF64<T> {
    fn value(&self) -> f64;
    fn new(value: f64) -> T;
}

#[allow(dead_code)]
trait UOMVectorMagnitudeF32<T: Clone + GetValueF32<T>>
where
    T: uom::num_traits::Num + Conversion<f32>,
{
    fn magnitude(&self) -> T;
}

impl<T: Clone + GetValueF32<T>> UOMVectorMagnitudeF32<T> for Vector2D<T>
where
    T: uom::num_traits::Num + Conversion<f32>,
{
    fn magnitude(&self) -> T {
        T::new((self.x.value().powi(2) + self.y.value().powi(2)).sqrt())
    }
}

#[allow(dead_code)]
trait UOMVectorMagnitudeF64<T: Clone + GetValueF64<T>>
where
    T: uom::num_traits::Num + Conversion<f64>,
{
    fn magnitude(&self) -> T;
}

impl<T: Clone + GetValueF64<T>> UOMVectorMagnitudeF64<T> for Vector2D<T>
where
    T: uom::num_traits::Num + Conversion<f64>,
{
    fn magnitude(&self) -> T {
        T::new((self.x.value().powi(2) + self.y.value().powi(2)).sqrt())
    }
}

impl<T: Clone> ToString for Vector2D<T>
where
    T: ToString,
{
    fn to_string(&self) -> String {
        format!("[{}, {}]", self.x.to_string(), self.y.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2d_arithmetic() {
        let vector1 = Vector2D { x: 1, y: 2 };
        let vector2 = Vector2D { x: 3, y: 4 };

        let vector3 = vector1.clone() + vector2.clone();
        assert_eq!(vector3.x, 4);
        assert_eq!(vector3.y, 6);

        let vector4 = vector1.clone() - vector2.clone();
        assert_eq!(vector4.x, -2);
        assert_eq!(vector4.y, -2);

        let vector5 = vector2d_mul!(vector1, 2);
        assert_eq!(vector5.x, 2);
        assert_eq!(vector5.y, 4);

        let vector6 = vector2d_div!(vector1, 2);
        assert_eq!(vector6.x, 0.5);
        assert_eq!(vector6.y, 1.0);
    }

    #[test]
    fn test_vector2d_magnitude() {
        let vector = Vector2D { x: 3, y: 4 };
        let magnitude = vector.magnitude();
        assert!((magnitude.value() - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_vector2d_products() {
        let vector1 = Vector2D { x: 1, y: 2 };
        let vector2 = Vector2D { x: 3, y: 4 };

        let dot = vector2d_dot!(vector1, vector2);
        assert_eq!(dot, 11);

        let cross = vector2d_cross!(vector1, vector2);
        assert_eq!(cross, -2);

        let hadamard = vector2d_hadamard!(vector1, vector2);
        assert_eq!(hadamard.x, 3);
        assert_eq!(hadamard.y, 8);

        let mul = vector2d_mul!(vector1, 2);
        assert_eq!(mul.x, 2);
        assert_eq!(mul.y, 4);
    }

    #[test]
    fn test_vector2d_to_string() {
        let vector = Vector2D { x: 1, y: 2 };
        let string = vector.to_string();
        assert_eq!(string, "[1, 2]");
    }

    fn test_vector2d_units() {
        let distance_x = uom::si::f32::Length::new::<uom::si::length::meter>(1.0);
        let distance_y = uom::si::f32::Length::new::<uom::si::length::meter>(2.0);

        let delta_position = Vector2D {
            x: distance_x,
            y: distance_y,
        };

        let time = uom::si::f32::Time::new::<uom::si::time::second>(1.0);

        let velocity = vector2d_div!(delta_position, time);

        let velocity_magnitude = velocity.magnitude();

        let velocity_string = velocity.to_string();
        let velocity_magnitude_string = velocity_magnitude.to_string();
    }
}
