use std::ops::{Add, Mul, Deref};
#[derive(Debug, PartialEq)]
pub struct Vector {
    data: Vec<f64>,
}
impl Deref for Vector {
    type Target = Vec<f64>;
    fn deref(&self) -> &Self::Target { &self.data }
}
// Allow comparisons between a Vector and integer/floating-point arrays
impl<const N: usize> PartialEq<[i32; N]> for Vector {
    fn eq(&self, other: &[i32; N]) -> bool {
        self.data.len() == N && self.data.iter().zip(other.iter())
            .all(|(a, &b)| *a == b as f64)
    }
}
impl<const N: usize> PartialEq<[f64; N]> for Vector {
    fn eq(&self, other: &[f64; N]) -> bool {
        self.data.len() == N && self.data.iter().zip(other.iter()).all(|(a, &b)| (*a - b).abs() < f64::EPSILON)
    }
}
// Allow comparisons between a Vector and Rust's Vec<...>
impl PartialEq<Vec<i32>> for Vector {
    fn eq(&self, other: &Vec<i32>) -> bool {
        self.data.iter().zip(other.iter()).all(|(a, &b)| *a == b as f64)
    }
}
impl PartialEq<Vec<f64>> for Vector {
    fn eq(&self, other: &Vec<f64>) -> bool {
        self.data.iter().zip(other.iter()).all(|(a, b)| (*a - *b).abs() < f64::EPSILON)
    }
}


// impl Mul<NumericalArgument> for Vector {
//     type Output = Self;
//
//     fn mul(self, rhs: NumericalArgument) -> Self::Output {
//         let n = rhs.into();
//         Self { data: self.data.iter().map(|v| v * (n as f64)).collect() }
//
//     }
// }
enum NumericalArgument {
    Int(i32),
    Float(f64),
}
impl From<i32> for NumericalArgument {
    fn from(n: i32) -> Self { NumericalArgument::Int(n) }
}
impl From<f64> for NumericalArgument {
    fn from(n: f64) -> Self { NumericalArgument::Float(n) }
}

impl Vector {
    pub fn new<T: Into<f64> + Copy, const N: usize>(data: [T; N]) -> Vector {
        Vector { data: data.iter().map(|&x| x.into()).collect() }
    }
    pub fn from_slice<T: Into<f64> + Copy>(data: &[T]) -> Vector {
        Vector { data: data.iter().map(|&x| x.into()).collect() }
    }
    pub fn pow<T: Into<NumericalArgument>>(&self, n: T) -> Vector {
        let n = n.into(); // automatically convert the argument to NumericalArgument
        let data = match  n {
            NumericalArgument::Int(n) => self.data.iter().map(|&v| v.powi(n as i32)).collect(),
            NumericalArgument::Float(n) => self.data.iter().map(|&v| v.powf(n as f64)).collect(),
        };
        Vector { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::vector::Vector;

    #[test]
    fn test_vector_array_equality() {
        let x = Vector::new([1, 2, 3]);
        assert_eq!( x, [1, 2, 3]);
        assert_eq!( x, [1.0, 2.0, 3.0]);
    }
    #[test]
    fn test_vector_vec_equality() {
        let x = Vector::new([1, 2, 3]);
        assert_eq!(x, vec![1, 2, 3]);
        assert_eq!( x, vec![1.0, 2.0, 3.0]);
    }
    #[test]
    fn test_vector_scalar_power() {
        let x = Vector::new([1, 2, 3]);
        let y = x.pow(3);
        assert_eq!( y, [1, 8, 27]);
        assert_eq!( y, [1.0, 8.0, 27.0]);

        let z = x.pow(0.5);
        assert_eq!( z, [x[0].sqrt(), x[1].sqrt(), x[2].sqrt()]);
    }
}