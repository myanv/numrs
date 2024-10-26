//
// pub fn lagrange<F>(x_input: Vec<f64>, y_input: Vec<f64>) -> F where F: Fn(f64) -> f64 {
//     move |x: f64| {
//         let n = x_input.len();
//
//     }
//
// }
//
// pub fn lagrange_basis<F>(x_input: &Vec<f64>, k: i16) -> impl Fn(f64) -> f64 {
//     move |x: f64| {
//         let n = x_input.len();
//
//         if k > n as i16 { Err("Index cannot be larger than the length of the input vector").expect("Index out of range"); }
//
//         let mut result = 1.0;
//         for i in 0..n {
//             if (i != k as usize) {
//                 result *= (x - x_input[i]) / (x_input[i] - x_input[k]);
//             }
//         }
//         result
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::vector::power::Vector;
//
//     #[test]
//     fn test_lagrange_basis() {
//         // let x = Vector::new([1, 2, 3]);
//         // let y = &x**3;
//         // assert_eq!( lagrange_basis(&x, 0), 1.0);
//         // assert_eq!( lagrange_basis(&x, 1), 8.0);
//     }
// }