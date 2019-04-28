pub fn solve() -> u64 {
    // Closed-form solution obtained from Binet's formula
    let phi = (1. + f64::sqrt(5.0)) / 2.;
    ((999. + f64::sqrt(5.0).log10()) / phi.log10()).ceil() as u64
}
