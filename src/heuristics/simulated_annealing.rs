use crate::solution::{LocalMove, Solution};

// TODO add some mechanism to auto save best every so often
/// Simulated annealing algorithm, automatically determines temperature
pub fn simulated_annealing<M, T>(solution: &mut T, num_iterations: u32)
where
    M: LocalMove<T>,
    T: Solution,
{
    // Determine temperature

    // Main loop
}

fn metropolis_rule() -> f64 {
    //...
    0.0
}
