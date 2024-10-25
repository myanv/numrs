pub mod bisection;
pub mod newton;

pub enum RootFindingTolerance {
    Empty,
    Is(f64)
}

pub fn get_tolerance(tol: &RootFindingTolerance) -> f64 {
    match tol {
        RootFindingTolerance::Empty => 0.0,
        RootFindingTolerance::Is(value) => *value,
    }
}