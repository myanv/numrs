use std::ops::Deref;
#[derive(Debug)]
pub struct Vector {
    data: Vec<f64>,
}
impl Deref for Vector {
    type Target = Vec<f64>;
    fn deref(&self) -> &Self::Target { &self.data }
}
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
        Vector {
            data: data.iter().map(|&x| x.into()).collect(),
        }
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
