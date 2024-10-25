use std::ops::Mul;
use crate::numrs_optimize::root_finding::{get_tolerance, RootFindingTolerance};


pub fn bisection<F>(f: F, mut a: f64, mut b: f64, tol: RootFindingTolerance) -> Result<f64, &'static str>
where
    F: Fn(f64) -> f64,
{
    if f(a) * f(b) > 0.0 {
        Err::<f64, &str>("f(a) and f(b) need to be opposite signs!").expect("Invalid arguments");
    }

    let tolerance = get_tolerance(&tol);

    while (a - b).abs() > 2.0 * tolerance {
        let m = (a + b) / 2.0;
        if f(m) == 0.0 {
            return Ok(m);
        } else if f(a) * f(m) < 0.0 {
            b = m;
        } else {
            a = m;
        }
    }
    println!("{:?}", (a + b) / 2.0);

    Ok((a + b) / 2.0)
}

#[cfg(test)]
mod tests {
    use crate::numrs_optimize::root_finding::bisection::*;

    #[test]
    fn test_bisection() {
        let double = |x| x * 2.0;
        assert_eq!(bisection(double, 2.0, 0.0, RootFindingTolerance::Empty), Ok(0.0));
        assert_eq!(bisection(double, 2.0, 0.0, RootFindingTolerance::Is(10e-5)), Ok(6.103515625e-5));
    }

    #[test]
    #[should_panic(expected = "Invalid arguments")]
    fn test_bisection_panic() {
        let double = |x| x * 2.0;
        bisection(double, 2.0, 4.0, RootFindingTolerance::Is(10e-5)).expect("Invalid arguments");
    }
}
