use cantor::matrix::Matrix;

fn main() {
    let x = Matrix::<i32, 4, 4>::new([
        2, 3, 4, 5,
        6, 7, 8, 9,
        10, 12, 13, 14,
        15, 16, 12, 13
    ]);

    println!("{}", x);
}