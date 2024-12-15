use std::ops::{Add, Mul, Sub};

use crate::zero::Zero;

pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> Vector<T, N>
where
    [(); N]: Sized,
{
    /// Creates a new `Vector` with the given array.
    pub fn new(values: [T; N]) -> Self {
        Vector(values)
    }

    /// Returns a zero-initialized `Vector` (if `T` implements `Zero`).
    pub fn zero() -> Self
    where
        T: Zero + Copy,
    {
        Vector([T::ZERO; N])
    }

    /// Creates a `Vector` where all elements are equal to `value`.
    pub fn fill(value: T) -> Self
    where
        T: Copy,
    {
        Vector([value; N])
    }

    /// Computes the dot product of two vectors.
    pub fn dot(&self, other: &Self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Zero + Copy,
    {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(T::ZERO, |acc, (&a, &b)| acc + a * b)
    }

    /// Returns the length (norm) of the vector.
    pub fn norm(&self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Zero + Copy + Into<f64> + From<f64>,
    {
        let sum_of_squares: f64 = self
            .0
            .iter()
            .map(|&x| (x * x).into())
            .fold(0.0, |acc, val| acc + val);
        sum_of_squares.sqrt().into()
    }
}

/// Addition of two vectors.
impl<T, const N: usize> Add for Vector<T, N>
where
    T: Add<Output = T> + Copy,
    [(); N]: Sized,
{
    type Output = Vector<T, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = [self.0[0]; N]; // Initialize with dummy value
        for i in 0..N {
            result[i] = self.0[i] + rhs.0[i];
        }
        Vector(result)
    }
}

/// Subtraction of two vectors.
impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Sub<Output = T> + Copy,
    [(); N]: Sized,
{
    type Output = Vector<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = [self.0[0]; N]; // Initialize with dummy value
        for i in 0..N {
            result[i] = self.0[i] - rhs.0[i];
        }
        Vector(result)
    }
}

/// Scalar multiplication.
impl<T, const N: usize> Mul<T> for Vector<T, N>
where
    T: Mul<Output = T> + Copy,
    [(); N]: Sized,
{
    type Output = Vector<T, N>;

    fn mul(self, scalar: T) -> Self::Output {
        let mut result = [self.0[0]; N]; // Initialize with dummy value
        for i in 0..N {
            result[i] = self.0[i] * scalar;
        }
        Vector(result)
    }
}

/// Display trait for printing.
impl<T, const N: usize> std::fmt::Debug for Vector<T, N>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Vector").field(&self.0).finish()
    }
}
