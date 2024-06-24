use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vector2D<T: Clone> {
    pub x: T,
    pub y: T,
}

impl<T: Clone> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<TLhs: Clone, TRhs: Clone> Add<Vector2D<TRhs>> for Vector2D<TLhs>
where
    TLhs: Add<TRhs>,
    <TLhs as Add<TRhs>>::Output: Clone,
{
    type Output = Vector2D<<TLhs as Add<TRhs>>::Output>;

    fn add(self, rhs: Vector2D<TRhs>) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<TLhs: Clone, TRhs: Clone> Sub<Vector2D<TRhs>> for Vector2D<TLhs>
where
    TLhs: Sub<TRhs>,
    <TLhs as Sub<TRhs>>::Output: Clone,
{
    type Output = Vector2D<<TLhs as Sub<TRhs>>::Output>;

    fn sub(self, rhs: Vector2D<TRhs>) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<TLhs: Clone, TRhs: Clone> Mul<TRhs> for Vector2D<TLhs>
where
    TLhs: Mul<TRhs>,
    <TLhs as Mul<TRhs>>::Output: Clone,
{
    type Output = Vector2D<<TLhs as Mul<TRhs>>::Output>;

    fn mul(self, rhs: TRhs) -> Self::Output {
        Vector2D {
            x: self.x * rhs.clone(),
            y: self.y * rhs.clone(),
        }
    }
}

impl<TLhs: Clone, TRhs: Clone> Div<TRhs> for Vector2D<TLhs>
where
    TLhs: Div<TRhs>,
    <TLhs as Div<TRhs>>::Output: Clone,
{
    type Output = Vector2D<<TLhs as Div<TRhs>>::Output>;

    fn div(self, rhs: TRhs) -> Self::Output {
        Vector2D {
            x: self.x / rhs.clone(),
            y: self.y / rhs.clone(),
        }
    }
}

impl<TLhs: Clone, TRhs: Clone> Rem<TRhs> for Vector2D<TLhs>
where
    TLhs: Rem<TRhs>,
    <TLhs as Rem<TRhs>>::Output: Clone,
{
    type Output = Vector2D<<TLhs as Rem<TRhs>>::Output>;

    fn rem(self, rhs: TRhs) -> Self::Output {
        Vector2D {
            x: self.x % rhs.clone(),
            y: self.y % rhs.clone(),
        }
    }
}

impl<T: Clone> Neg for Vector2D<T>
where
    T: Neg<Output = T>,
{
    type Output = Vector2D<T>;

    fn neg(self) -> Self::Output {
        Vector2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Clone> Index<usize> for Vector2D<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T: Clone + AddAssign> AddAssign for Vector2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Clone + SubAssign> SubAssign for Vector2D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<TLhs: Clone + MulAssign, TRhs: Clone> MulAssign<TRhs> for Vector2D<TLhs>
where
    TLhs: MulAssign<TRhs>,
{
    fn mul_assign(&mut self, rhs: TRhs) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
    }
}

impl<TLhs: Clone + DivAssign, TRhs: Clone> DivAssign<TRhs> for Vector2D<TLhs>
where
    TLhs: DivAssign<TRhs>,
{
    fn div_assign(&mut self, rhs: TRhs) {
        self.x /= rhs.clone();
        self.y /= rhs.clone();
    }
}

impl<T: Clone + RemAssign> RemAssign for Vector2D<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

impl<T: Clone> IndexMut<usize> for Vector2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

pub trait Vector2DOperations<TRhs: Clone> {
    type OutputCrossAndDot;
    type OutputHadamard;

    fn dot(self, rhs: Vector2D<TRhs>) -> Self::OutputCrossAndDot;
    fn cross(self, rhs: Vector2D<TRhs>) -> Self::OutputCrossAndDot;
    fn hadamard(self, rhs: Vector2D<TRhs>) -> Self::OutputHadamard;
}

impl<TLhs: Clone, TRhs: Clone> Vector2DOperations<TRhs> for Vector2D<TLhs>
where
    TLhs: Mul<TRhs>,
    <TLhs as Mul<TRhs>>::Output: Clone
        + Add<Output = <TLhs as Mul<TRhs>>::Output>
        + Sub<Output = <TLhs as Mul<TRhs>>::Output>,
{
    type OutputCrossAndDot = <TLhs as Mul<TRhs>>::Output;
    type OutputHadamard = Vector2D<<TLhs as Mul<TRhs>>::Output>;

    fn dot(self, rhs: Vector2D<TRhs>) -> Self::OutputCrossAndDot {
        self.x * rhs.x + self.y * rhs.y
    }

    fn cross(self, rhs: Vector2D<TRhs>) -> Self::OutputCrossAndDot {
        self.x * rhs.y - self.y * rhs.x
    }

    fn hadamard(self, rhs: Vector2D<TRhs>) -> Self::OutputHadamard {
        Vector2D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

// all the vector products, dot, cross, hadamard

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::f64::*;
    use uom::si::*;

    #[test]
    fn test_arithmetic() {
        let five_meters = Length::new::<length::meter>(5.0);
        let two_inches = Length::new::<length::inch>(2.0);
        let four_seconds = Time::new::<time::second>(4.0);
        let eight = 8.0f64;

        let v1 = Vector2D::new(five_meters, two_inches);
        let v2 = Vector2D::new(five_meters * 2.0, two_inches * 5.0);

        let _v3 = v1.clone() + v2.clone();

        let v4 = v1.clone() / four_seconds.clone();

        let _v5 = v1.clone() * eight;

        let mut v6 = v4.clone();
        let v7 = v6.clone() * eight;

        v6 += v7;

        v6[0] = v6[1].clone();

        dbg!(v1);
        dbg!(v2);
        dbg!(v4);
        dbg!(v7);
        dbg!(v6);

        let _dot_res = v1.clone().dot(v2.clone());
        let _hadamard_res = v1.clone().hadamard(v2.clone());
        let _cross_res = v1.clone().cross(v2.clone());

        dbg!(_dot_res);
        dbg!(_hadamard_res);
        dbg!(_cross_res);
    }
}
