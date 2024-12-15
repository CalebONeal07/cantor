#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

pub mod matrix;
mod one;
mod vector;
mod zero;

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn matrix_addition() {
        let a: Matrix<f32, 3, 3> = Matrix::zeroes();
        let b: Matrix<f32, 3, 3> = Matrix::new([2.0, 3.0, 4.0, 4.0, 5.0, 6.0, 5.0, 7.0, 9.0]);

        assert!(a * b == a)
    }

    #[test]
    fn matrix_subtraction() {}
}
