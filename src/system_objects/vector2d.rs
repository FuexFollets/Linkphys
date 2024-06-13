use std::ops::{Add, Sub};

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

#[macro_export]
macro_rules! vector2d_magnitude {
    ($lhs:expr) => {
        ($lhs.x.clone() * $lhs.x.clone() + $lhs.y.clone() * $lhs.y.clone()).sqrt()
    };
}

pub struct Vector2D<T: Clone> {
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

/*
#[allow(dead_code)]
trait UOMVectorMagnitudeF32<T: Clone + GetValueF32<T>>
where
    T: uom::num_traits::Num + Conversion<f32>,
{
    fn magnitude(&self) -> T;
}

trait GetValueF32<T> {
    fn value(&self) -> f32;
    fn new(value: f32) -> T;
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

trait GetValueF64<T> {
    fn value(&self) -> f64;
    fn new(value: f64) -> T;
}

impl<T: Clone + GetValueF64<T>> UOMVectorMagnitudeF64<T> for Vector2D<T>
where
    T: uom::num_traits::Num + Conversion<f64>,
{
    fn magnitude(&self) -> T {
        T::new((self.x.value().powi(2) + self.y.value().powi(2)).sqrt())
    }
}

#[allow(dead_code)]
trait VectorMagnitude<T> {
    fn magnitude(&self) -> T;
}

impl VectorMagnitude<f32> for Vector2D<f32> {
    fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl VectorMagnitude<f64> for Vector2D<f64> {
    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
*/

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
        let vector1 = Vector2D {
            x: 1.0f32,
            y: 2.0f32,
        };
        let vector2 = Vector2D {
            x: 3.0f32,
            y: 4.0f32,
        };

        let vector3 = vector1.clone() + vector2.clone();
        assert!((vector3.x - 4.0f32).abs() < 1e-5);
        assert!((vector3.y - 6.0f32).abs() < 1e-5);

        let vector4 = vector1.clone() - vector2.clone();
        assert!((vector4.x + 2.0f32).abs() < 1e-5);
        assert!((vector4.y + 2.0f32).abs() < 1e-5);

        let vector5 = vector2d_mul!(vector1, 2.0f32);
        assert!((vector5.x - 2.0f32).abs() < 1e-5);
        assert!((vector5.y - 4.0f32).abs() < 1e-5);

        let vector6 = vector2d_div!(vector1, 2.0f32);
        assert!((vector6.x - 0.5f32).abs() < 1e-5);
        assert!((vector6.y - 1.0f32).abs() < 1e-5);
    }

    #[test]
    fn test_vector2d_magnitude() {
        let vector = Vector2D {
            x: 3.0f32,
            y: 4.0f32,
        };

        let magnitude = vector2d_magnitude!(vector);
        assert!((magnitude - 5.0f32).abs() < 1e-5);
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

    #[test]
    fn test_vector2d_units() {
        let distance_x = uom::si::f32::Length::new::<uom::si::length::meter>(1.0f32);
        let distance_y = uom::si::f32::Length::new::<uom::si::length::meter>(2.0f32);

        let delta_position = Vector2D {
            x: distance_x,
            y: distance_y,
        };

        let time = uom::si::f32::Time::new::<uom::si::time::second>(1.0f32);

        let velocity = vector2d_div!(delta_position, time);

        let velocity_magnitude = vector2d_magnitude!(velocity);

        assert!((velocity_magnitude.value - 2.23606797749979f32).abs() < 1e-5f32);
    }
}
