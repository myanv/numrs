use crate::numrs_optimize::root_finding::{get_tolerance, RootFindingTolerance};


// TODO: Expand to use Newton-Raphson's method if fprime is provided, use Halley's method if fprime2 is provided. Default is secant
// Similar to scipy.optimize.newton: https://docs.scipy.org/doc/scipy/reference/generated/scipy.optimize.newton.html

pub fn secant<F>(f: F, x0: f64, x1: f64, tol: RootFindingTolerance) -> Result<f64, &'static str>
where
    F: Fn(f64) -> f64,
{
    if f(x1) - f(x0) == 0.0 {
        Err::<f64, &'static str>("Division by zero").expect("Error:");
    }

    let mut x_prev = x0;
    let mut x_current = x1;
    let mut x_next = x_current - f(x_current) * ( (x_current - x_prev) / (f(x_current) - f(x_prev)) );

    let tolerance = get_tolerance(&tol);

    while x_next.abs() > 2.0 * tolerance {
        x_prev = x_current;
        x_current = x_next;
        x_next = x_current - f(x_current) * ( (x_current - x_prev) / (f(x_current) - f(x_prev)) );
    }
    Ok(x_next)
}

#[cfg(test)]
mod test {
    use crate::numrs_optimize::root_finding::RootFindingTolerance;
    use crate::numrs_optimize::root_finding::newton::secant;

    #[test]
    fn test_secant() {
        let double = |x| x * 2.0;
        assert_eq!(secant(double, -2.0, -1.0, RootFindingTolerance::Empty), Ok(0.0));
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_secant_division_by_zero() {
        let double = |x| x * 2.0;
        secant(double, 0.0, 0.0, RootFindingTolerance::Empty).expect("Division by zero");
    }
}