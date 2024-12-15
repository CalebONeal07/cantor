use crate::one::One;
use crate::zero::Zero;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Eq)]
pub struct Matrix<T, const ROWS: usize, const COLUMNS: usize>([T; ROWS * COLUMNS])
where
    [(); ROWS * COLUMNS]: Sized;

impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]: Sized,
{
    pub fn new(vals: [T; ROWS * COLUMNS]) -> Self {
        debug_assert_eq!(vals.len(), ROWS * COLUMNS);
        Matrix(vals)
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]: Sized,
    [(); COLUMNS * ROWS]: Sized,
    T: Copy,
{
    pub fn transpose(&self) -> Matrix<T, COLUMNS, ROWS> {
        let mut transposed = [self.0[0].clone(); COLUMNS * ROWS];

        for row in 0..ROWS {
            for col in 0..COLUMNS {
                transposed[col * ROWS + row] = self.0[row * COLUMNS + col].clone();
            }
        }

        Matrix(transposed)
    }
}

impl<T, const N: usize> Matrix<T, N, N>
where
    [(); N * N]: Sized,
{
    pub fn transpose_in_place(&mut self) {
        for row in 0..N {
            for col in (row + 1)..N {
                self.0.swap(row * N + col, col * N + row);
            }
        }
    }
}

impl<T, const N: usize> Matrix<T, N, N>
where
    T: Zero + One + Copy,
    [(); N * N]: Sized,
{
    pub fn identity() -> Matrix<T, N, N> {
        let mut identity = [T::ZERO; N * N];

        for i in 0..N {
            identity[i * (N + 1)] = T::one();
        }

        Matrix(identity)
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    T: Zero + Copy,
    [(); ROWS * COLUMNS]: Sized,
{
    pub fn zeroes() -> Self {
        Matrix([T::ZERO; ROWS * COLUMNS])
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T, ROWS, COLUMNS>
where
    T: One + Copy,
    [(); ROWS * COLUMNS]: Sized,
{
    pub fn ones() -> Self {
        Matrix([T::one(); ROWS * COLUMNS])
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Display for Matrix<T, ROWS, COLUMNS>
where
    T: Display,
    [(); ROWS * COLUMNS]: Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..ROWS {
            write!(f, "[")?;
            for col in 0..COLUMNS {
                write!(f, "{}", self.0[row * COLUMNS + col])?;
                if col < COLUMNS - 1 {
                    write!(f, " ")?; // Add a space between elements
                }
            }
            write!(f, "]")?;
            if row < ROWS - 1 {
                writeln!(f)?; // Add a newline between rows
            }
        }
        Ok(())
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Add for Matrix<T, ROWS, COLUMNS>
where
    T: Add<T, Output = T> + Copy,
    [(); ROWS * COLUMNS]: Sized,
{
    type Output = Matrix<T, ROWS, COLUMNS>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result: [T; ROWS * COLUMNS] = self.0;
        for i in 0..ROWS * COLUMNS {
            result[i] = result[i].clone() + rhs.0[i].clone();
        }

        Matrix(result)
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> AddAssign for Matrix<T, ROWS, COLUMNS>
where
    T: AddAssign<T> + Copy,
    [(); ROWS * COLUMNS]: Sized,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..ROWS * COLUMNS {
            self.0[i] += rhs.0[i].clone();
        }
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Sub for Matrix<T, ROWS, COLUMNS>
where
    T: Sub<T, Output = T> + Copy,
    [(); ROWS * COLUMNS]: Sized,
{
    type Output = Matrix<T, ROWS, COLUMNS>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result: [T; ROWS * COLUMNS] = self.0;
        for i in 0..ROWS * COLUMNS {
            result[i] = result[i].clone() - rhs.0[i].clone();
        }
        Matrix(result)
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> SubAssign for Matrix<T, ROWS, COLUMNS>
where
    T: SubAssign<T> + Copy,
    [(); ROWS * COLUMNS]: Sized,
{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..ROWS * COLUMNS {
            self.0[i] -= rhs.0[i].clone();
        }
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize, const N: usize> Mul<Matrix<T, COLUMNS, N>>
    for Matrix<T, ROWS, COLUMNS>
where
    T: Add<T, Output = T> + Mul<T, Output = T> + Copy + Zero,
    [(); ROWS * COLUMNS]: Sized,
    [(); COLUMNS * N]: Sized,
    [(); ROWS * N]: Sized,
{
    type Output = Matrix<T, ROWS, N>;

    fn mul(self, rhs: Matrix<T, COLUMNS, N>) -> Self::Output {
        let mut result = [T::ZERO; ROWS * N];

        for row in 0..ROWS {
            for col in 0..N {
                let mut sum = T::ZERO;
                for k in 0..COLUMNS {
                    sum = sum + self.0[row * COLUMNS + k] * rhs.0[k * N + col];
                }
                result[row * N + col] = sum;
            }
        }

        Matrix(result)
    }
}

impl<T, const N: usize> MulAssign<Matrix<T, N, N>> for Matrix<T, N, N>
where
    T: Add<T, Output = T> + Mul<T, Output = T> + Copy + Zero,
    [(); N * N]: Sized,
{
    fn mul_assign(&mut self, rhs: Matrix<T, N, N>) {
        let mut result = [T::ZERO; N * N];

        for row in 0..N {
            for col in 0..N {
                let mut sum = T::ZERO;
                for k in 0..N {
                    sum = sum + self.0[row * N + k] * rhs.0[k * N + col];
                }
                result[row * N + col] = sum;
            }
        }

        // Copy the result back into self
        self.0.copy_from_slice(&result);
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Index<(usize, usize)> for Matrix<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]: Sized,
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        debug_assert!(row < ROWS, "Row index out of bounds");
        debug_assert!(col < COLUMNS, "Column index out of bounds");
        &self.0[row * COLUMNS + col]
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> IndexMut<(usize, usize)>
    for Matrix<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]: Sized,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        debug_assert!(row < ROWS, "Row index out of bounds");
        debug_assert!(col < COLUMNS, "Column index out of bounds");
        &mut self.0[row * COLUMNS + col]
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> Mul<T> for Matrix<T, ROWS, COLUMNS>
where
    T: Clone + Copy,
    for<'a> &'a T: MulAssign<T>,
    [(); ROWS * COLUMNS]: Sized,
{
    type Output = Matrix<T, ROWS, COLUMNS>;

    fn mul(self, rhs: T) -> Self::Output {
        let result = self.0.clone();

        result.iter().for_each(|mut x| x *= rhs);

        Matrix(result)
    }
}

impl<T, const ROWS: usize, const COLUMNS: usize> PartialEq for Matrix<T, ROWS, COLUMNS>
where
    T: PartialEq,
    [(); ROWS * COLUMNS]: Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
