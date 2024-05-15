use crate::solution::{LocalMove, Solution};

// TODO add some mechanism to auto save best every so often
/// Simulated annealing algorithm, automatically determines temperature.
/// Cools after every iteration.
pub fn simulated_annealing<M, T>(
    solution: &mut T,
    smallest_cost_difference: f64,
    num_iterations: u32,
    num_iterations_temperature_determining: u32,
    cooling_schedule: CoolingSchedule,
) where
    M: LocalMove<T>,
    T: Solution,
{
    // Determine starting temperature by doing random moves and checking variance in solution cost
    let mut old_cost;
    let mut total_cost_diff = 0.0;
    for _ in 0..num_iterations_temperature_determining {
        old_cost = solution.get_cost();
        M::do_random_move(solution);
        let cost_diff = solution.get_cost() - old_cost;
        total_cost_diff += cost_diff.abs();
    }
    let avg_cost_diff = total_cost_diff / num_iterations_temperature_determining as f64;
    let starting_accepting_probability: f64 = 0.5; // 0.2 is better for a greeding starting solution; todo?
    let starting_temperature = -avg_cost_diff / starting_accepting_probability.ln();
    let ending_accepting_probability: f64 = 0.0000000001;
    let ending_temperature = -smallest_cost_difference / ending_accepting_probability.ln();

    // Main loop
    let mut temperature = starting_temperature;
    // todo: determine cooling schedule based on input
    let mut temperature = starting_temperature;

    println!("Starting temperature: {}, avg cost diff {}", starting_temperature, avg_cost_diff);
    println!("Ending temperature: {}", ending_temperature);

}

fn metropolis_rule() -> f64 {
    //...
    0.0
}

pub enum CoolingSchedule {
    /// New temp = old temp - c for constant c > 0
    Linear,
    /// New temp = old temp * c for constant 0 < c < 1
    Geometric,
}
