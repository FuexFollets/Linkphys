use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

use std::fmt::Display;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct VectorND<T: Clone, const N: usize> {
    pub data: [T; N],
}

impl<T: Clone, const N: usize> VectorND<T, N> {
    pub fn new(data: [T; N]) -> Self {
        VectorND { data }
    }
}

impl<TLhs: Clone, TRhs: Clone, const N: usize> Add<VectorND<TRhs, N>> for VectorND<TLhs, N>
where
    TLhs: Add<TRhs>,
    <TLhs as Add<TRhs>>::Output: Clone + Copy + Default,
{
    type Output = VectorND<<TLhs as Add<TRhs>>::Output, N>;

    fn add(self, rhs: VectorND<TRhs, N>) -> Self::Output {
        let mut new_data = [<TLhs as Add<TRhs>>::Output::default(); N];

        for index in 0..N {
            new_data[index] = self.data[index].clone() + rhs.data[index].clone();
        }

        VectorND { data: new_data }
    }
}

impl<TLhs: Clone, TRhs: Clone, const N: usize> Sub<VectorND<TRhs, N>> for VectorND<TLhs, N>
where
    TLhs: Sub<TRhs>,
    <TLhs as Sub<TRhs>>::Output: Clone + Copy + Default,
{
    type Output = VectorND<<TLhs as Sub<TRhs>>::Output, N>;

    fn sub(self, rhs: VectorND<TRhs, N>) -> Self::Output {
        let mut new_data = [<TLhs as Sub<TRhs>>::Output::default(); N];

        for index in 0..N {
            new_data[index] = self.data[index].clone() - rhs.data[index].clone();
        }

        VectorND { data: new_data }
    }
}

impl<TLhs: Clone, TRhs: Clone, const N: usize> Mul<TRhs> for VectorND<TLhs, N>
where
    TLhs: Mul<TRhs>,
    <TLhs as Mul<TRhs>>::Output: Clone + Copy + Default,
{
    type Output = VectorND<<TLhs as Mul<TRhs>>::Output, N>;

    fn mul(self, rhs: TRhs) -> Self::Output {
        let mut new_data = [<TLhs as Mul<TRhs>>::Output::default(); N];

        for index in 0..N {
            new_data[index] = self.data[index].clone() * rhs.clone();
        }

        VectorND { data: new_data }
    }
}

impl<TLhs: Clone, TRhs: Clone, const N: usize> Div<TRhs> for VectorND<TLhs, N>
where
    TLhs: Div<TRhs>,
    <TLhs as Div<TRhs>>::Output: Clone + Copy + Default,
{
    type Output = VectorND<<TLhs as Div<TRhs>>::Output, N>;

    fn div(self, rhs: TRhs) -> Self::Output {
        let mut new_data = [<TLhs as Div<TRhs>>::Output::default(); N];

        for index in 0..N {
            new_data[index] = self.data[index].clone() / rhs.clone();
        }

        VectorND { data: new_data }
    }
}

impl<TLhs: Clone, TRhs: Clone, const N: usize> Rem<TRhs> for VectorND<TLhs, N>
where
    TLhs: Rem<TRhs>,
    <TLhs as Rem<TRhs>>::Output: Clone + Copy + Default,
{
    type Output = VectorND<<TLhs as Rem<TRhs>>::Output, N>;

    fn rem(self, rhs: TRhs) -> Self::Output {
        let mut new_data = [<TLhs as Rem<TRhs>>::Output::default(); N];

        for index in 0..N {
            new_data[index] = self.data[index].clone() % rhs.clone();
        }

        VectorND { data: new_data }
    }
}

impl<T: Clone, const N: usize> Neg for VectorND<T, N>
where
    T: Neg<Output = T>,
    <T as Neg>::Output: Clone + Copy + Default,
{
    type Output = VectorND<T, N>;

    fn neg(self) -> Self::Output {
        let mut new_data = [<T as Neg>::Output::default(); N];

        for index in 0..N {
            new_data[index] = -self.data[index].clone();
        }

        VectorND { data: new_data }
    }
}

impl<T: Clone, const N: usize> Index<usize> for VectorND<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Clone, const N: usize> IndexMut<usize> for VectorND<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Clone, const N: usize> Default for VectorND<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        VectorND {
            data: [T::default(); N],
        }
    }
}

impl<T: Clone, const N: usize> Display for VectorND<T, N>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for index in 0..N {
            write!(f, "{}", self.data[index])?;
            if index < N - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

// add assign

impl<TLhs, TRhs: Clone, const N: usize> AddAssign<VectorND<TRhs, N>> for VectorND<TLhs, N>
where
    TLhs: AddAssign<TRhs> + Clone,
{
    fn add_assign(&mut self, rhs: VectorND<TRhs, N>) {
        for index in 0..N {
            self.data[index] += rhs.data[index].clone();
        }
    }
}

impl<TLhs, TRhs: Clone, const N: usize> SubAssign<VectorND<TRhs, N>> for VectorND<TLhs, N>
where
    TLhs: SubAssign<TRhs> + Clone,
{
    fn sub_assign(&mut self, rhs: VectorND<TRhs, N>) {
        for index in 0..N {
            self.data[index] -= rhs.data[index].clone();
        }
    }
}

impl<TLhs, TRhs: Clone, const N: usize> MulAssign<TRhs> for VectorND<TLhs, N>
where
    TLhs: MulAssign<TRhs> + Clone,
{
    fn mul_assign(&mut self, rhs: TRhs) {
        for index in 0..N {
            self.data[index] *= rhs.clone();
        }
    }
}

impl<TLhs, TRhs: Clone, const N: usize> DivAssign<TRhs> for VectorND<TLhs, N>
where
    TLhs: DivAssign<TRhs> + Clone,
{
    fn div_assign(&mut self, rhs: TRhs) {
        for index in 0..N {
            self.data[index] /= rhs.clone();
        }
    }
}

impl<TLhs, TRhs: Clone, const N: usize> RemAssign<TRhs> for VectorND<TLhs, N>
where
    TLhs: RemAssign<TRhs> + Clone,
{
    fn rem_assign(&mut self, rhs: TRhs) {
        for index in 0..N {
            self.data[index] %= rhs.clone();
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MatrixNMD<T: Clone, const N: usize, const M: usize> {
    data: [[T; N]; M],
}

impl<T: Clone, const N: usize, const M: usize> MatrixNMD<T, N, M> {
    pub fn new(data: [[T; N]; M]) -> Self {
        MatrixNMD { data }
    }
}

impl<TLhs: Clone, TRhs: Clone, const N: usize, const M: usize> Add<MatrixNMD<TRhs, N, M>>
    for MatrixNMD<TLhs, N, M>
where
    TLhs: Add<TRhs>,
    <TLhs as Add<TRhs>>::Output: Clone + Copy + Default,
{
    type Output = MatrixNMD<<TLhs as Add<TRhs>>::Output, N, M>;

    fn add(self, rhs: MatrixNMD<TRhs, N, M>) -> Self::Output {
        let mut new_data = [[<TLhs as Add<TRhs>>::Output::default(); N]; M];

        for row in 0..M {
            for column in 0..N {
                new_data[row][column] =
                    self.data[row][column].clone() + rhs.data[row][column].clone();
            }
        }

        MatrixNMD::new(new_data)
    }
}

impl<TLhs: Clone, TRhs: Clone, const N: usize, const M: usize> AddAssign<MatrixNMD<TRhs, N, M>>
    for MatrixNMD<TLhs, N, M>
where
    TLhs: AddAssign<TRhs> + Clone,
{
    fn add_assign(&mut self, rhs: MatrixNMD<TRhs, N, M>) {
        for row in 0..M {
            for column in 0..N {
                self.data[row][column] += rhs.data[row][column].clone();
            }
        }
    }
}

//neg

impl<T: Clone, const N: usize, const M: usize> Neg for MatrixNMD<T, N, M>
where
    T: Neg<Output = T>,
    <T as Neg>::Output: Clone + Copy + Default,
{
    type Output = MatrixNMD<T, N, M>;

    fn neg(self) -> Self::Output {
        let mut new_data = [[<T as Neg>::Output::default(); N]; M];

        for row in 0..M {
            for column in 0..N {
                new_data[row][column] = -self.data[row][column].clone();
            }
        }

        MatrixNMD::new(new_data)
    }
}

// mul

impl<TLhs: Clone, TRhs: Clone, const N: usize, const M: usize> Mul<MatrixNMD<TRhs, N, M>>
    for MatrixNMD<TLhs, N, M>
where
    TLhs: Mul<TRhs>,
    <TLhs as Mul<TRhs>>::Output: Clone + Copy + Default,
{
    type Output = MatrixNMD<<TLhs as Mul<TRhs>>::Output, N, M>;

    fn mul(self, rhs: MatrixNMD<TRhs, N, M>) -> Self::Output {
        let mut new_data = [[<TLhs as Mul<TRhs>>::Output::default(); N]; M];

        for row in 0..M {
            for column in 0..N {
                new_data[row][column] =
                    self.data[row][column].clone() * rhs.data[row][column].clone();
            }
        }

        MatrixNMD::new(new_data)
    }
}

// div

impl<TLhs: Clone, TRhs: Clone, const N: usize, const M: usize> Div<MatrixNMD<TRhs, N, M>>
    for MatrixNMD<TLhs, N, M>
where
    TLhs: Div<TRhs>,
    <TLhs as Div<TRhs>>::Output: Clone + Copy + Default,
{
    type Output = MatrixNMD<<TLhs as Div<TRhs>>::Output, N, M>;

    fn div(self, rhs: MatrixNMD<TRhs, N, M>) -> Self::Output {
        let mut new_data = [[<TLhs as Div<TRhs>>::Output::default(); N]; M];

        for row in 0..M {
            for column in 0..N {
                new_data[row][column] =
                    self.data[row][column].clone() / rhs.data[row][column].clone();
            }
        }

        MatrixNMD::new(new_data)
    }
}

/*
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
*/

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