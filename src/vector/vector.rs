use std::ops::{Add, Mul, Deref};
#[derive(Debug, PartialEq)]
pub struct Vector {
    data: Vec<f64>,
}
impl Deref for Vector {
    type Target = Vec<f64>;
    fn deref(&self) -> &Self::Target { &self.data }
}

// Allow comparisons between a Vector and [1, 2, 3], [1.0, 2.0, 3.0] etc.
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
    fn test_vector() {
        let x = Vector::new([1, 2, 3]);
        assert_eq!( x, [1, 2, 3]);
        assert_eq!( x, vec![1, 2, 3]);
        assert_eq!( x, [1.0, 2.0, 3.0]);
    }
}