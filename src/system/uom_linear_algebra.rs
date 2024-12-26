use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

use std::fmt::Display;

use ndarray;

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

impl<TLhs: Clone, TRhs: Clone, const N: usize, const M: usize> MulAssign<MatrixNMD<TRhs, N, M>>
    for MatrixNMD<TLhs, N, M>
where
    TLhs: MulAssign<TRhs> + Clone,
{
    fn mul_assign(&mut self, rhs: MatrixNMD<TRhs, N, M>) {
        for row in 0..M {
            for column in 0..N {
                self.data[row][column] *= rhs.data[row][column].clone();
            }
        }
    }
}

impl<TLhs: Clone, TRhs: Clone, const N: usize, const M: usize> DivAssign<MatrixNMD<TRhs, N, M>>
    for MatrixNMD<TLhs, N, M>
where
    TLhs: DivAssign<TRhs> + Clone,
{
    fn div_assign(&mut self, rhs: MatrixNMD<TRhs, N, M>) {
        for row in 0..M {
            for column in 0..N {
                self.data[row][column] /= rhs.data[row][column].clone();
            }
        }
    }
}

impl<TLhs: Clone, TRhs: Clone, const N: usize, const M: usize> RemAssign<MatrixNMD<TRhs, N, M>>
    for MatrixNMD<TLhs, N, M>
where
    TLhs: RemAssign<TRhs> + Clone,
{
    fn rem_assign(&mut self, rhs: MatrixNMD<TRhs, N, M>) {
        for row in 0..M {
            for column in 0..N {
                self.data[row][column] %= rhs.data[row][column].clone();
            }
        }
    }
}

impl<T: Clone, const N: usize, const M: usize> Display for MatrixNMD<T, N, M>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..M {
            write!(f, "[")?;
            for column in 0..N {
                write!(f, "{}", self.data[row][column])?;
                if column < N - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]\n")?;
        }
        Ok(())
    }
}

// implementing products for vector and matrix (dot, cross, hadamard)

pub trait DotProduct<TRhs: Clone, const N: usize> {
    type OutputCrossAndDot;

    fn dot(self, rhs: VectorND<TRhs, N>) -> Self::OutputCrossAndDot;
}

impl<const N: std::vec<usize>> DotProduct<f64, N> for VectorND<f64, N> {
    
}

/*
impl<TLhs: Clone, TRhs: Clone, const N: usize> VectorProducts<TRhs, N> for VectorND<TLhs, N>
where
    <TLhs as Mul<TRhs>>::Output: Clone + Default,
    TLhs: Mul<TRhs>,
{
    type OutputMul = <TLhs as Mul<TRhs>>::Output;
    type OutputCrossAndDot = <TLhs as Mul<TRhs>>::Output;
    type OutputHadamard = VectorND<<TLhs as Mul<TRhs>>::Output, N>;

    fn dot(self, VectorND { data }: VectorND<TRhs, N>) -> Self::OutputCrossAndDot {}

    fn cross(self, rhs: VectorND<TRhs, N>) -> Self::OutputCrossAndDot {}

    fn hadamard(self, rhs: VectorND<TRhs, N>) -> Self::OutputHadamard {}
}
*/

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
    use ndarray::Dim;
    use uom::si::f64::*;
    use uom::si::*;

    #[test]
    fn test_ndarray() {
        type T = ndarray::Array<ndarray::OwnedRepr<[f64; 4]>, Dim<[usize; 0]>>;

        let values = [1.0, 2.0, 3.0, 4.0];

        let array = ndarray::arr0([values, values]);

        dbg!(array);
    }

    #[test]
    fn test_sum_uom() {
        let five_meters = Length::new::<length::meter>(5.0);
        let list = [five_meters, five_meters, five_meters, five_meters];

        let res = list.iter().fold(Length::default(), |acc, x| acc + *x);
    }

    #[test]
    fn test_arithmetic() {
        let five_meters = Length::new::<length::meter>(5.0);
        let two_inches = Length::new::<length::inch>(2.0);
        let four_seconds = Time::new::<time::second>(4.0);
        let eight = 8.0f64;

        let v1 = VectorND::new([five_meters, two_inches, five_meters * eight]);
        let v2 = VectorND::new([two_inches * 100.0, two_inches, two_inches * eight]);

        let res = (v1 - v2) * four_seconds / 10.0;
    }
}
