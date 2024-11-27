use std::ops::{Add, Sub, Mul, Div};

type Field<T> = Box<dyn Add<Output = T> + Sub<Output = T>  + Mul<Output = T>   + Div<Output = T>  >;