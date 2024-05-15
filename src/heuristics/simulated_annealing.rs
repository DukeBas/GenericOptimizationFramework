use crate::solution::{LocalMove, Solution};
use rand::{rngs::SmallRng, Rng, SeedableRng};

// TODO add some mechanism to auto save best every so often
/// Simulated annealing algorithm, automatically determines temperature.
/// Cools after every iteration.
/// TODO: currently only is for minimization problems, should be generalized?
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

    // Determine ending temperature
    let ending_accepting_probability: f64 = 0.0000000001; // todo extract in some way
    let ending_temperature = -smallest_cost_difference / ending_accepting_probability.ln();

    // Get the cooling schedule
    let cooling_schedule: Box<dyn Fn(f64) -> f64> = get_cooling_schedule(
        cooling_schedule,
        starting_temperature,
        ending_temperature,
        num_iterations,
    );

    // Print some info
    println!(
        "Running simulated annealing for {} iterations with starting temperature: {} and ending temperature: {}",
        num_iterations, starting_temperature, ending_temperature
    );

    // Main loop
    let mut temperature = starting_temperature;
    let mut previous_cost = solution.get_cost();
    let mut small_rng = SmallRng::from_entropy();
    let mut best_solution = solution.clone();
    for it in 0..num_iterations {
        // Do the move
        M::do_random_move(solution);

        // Check new cost again the one of the previous iteration
        let mut new_cost = solution.get_cost();
        let cost_diff = previous_cost - new_cost;

        // Check if we accept the move, always accept if it is better
        if cost_diff < 0.0 {
            // Maybe reject the move
            // Get a random number in [0, 1)
            let random_number: f64 = small_rng.gen();
            if random_number > metropolis_rule(cost_diff, temperature) {
                // Reject the move, undo it
                M::undo_last_move(solution);
                new_cost = previous_cost;
                // println!("Rejected move with cost diff: {}, old {}, new {}", cost_diff, previous_cost, new_cost);
            } else {
                // println!("Accepted worse move with cost diff: {}, old {}, new {}", cost_diff, previous_cost, new_cost);
            }
            
        } else {
            // Always accept if it is better
            // println!("Accepted better move with cost diff: {}, old {}, new {}", cost_diff, previous_cost, new_cost);
        }

        // Update temperature
        temperature = cooling_schedule(temperature);

        // Update previous cost
        previous_cost = new_cost;

        // print cost every so often
        if it % 10000 == 0 {
            println!("Cost: {} temp {}", solution.get_cost(), temperature);
        }

        // Update best solution
        if solution.get_cost() < best_solution.get_cost() {
            best_solution = solution.clone();
        }
    }

    // Set the best solution
    *solution = best_solution;

    // Print final cost
    println!("Final cost: {}", solution.get_cost());
}

/// Metropolis rule for simulated annealing. Assumes cost difference is negative.
#[inline(always)]
fn metropolis_rule(cost_difference: f64, temperature: f64) -> f64 {
    // println!("metro {}", ((cost_difference).exp() / temperature)); // TODO: rescale based on problem? !!! use avg difference?
    (cost_difference).exp() / temperature
}

fn get_cooling_schedule(
    cooling_schedule: CoolingSchedule,
    start_temperature: f64,
    end_temperature: f64,
    num_iterations: u32,
) -> Box<dyn Fn(f64) -> f64> {
    match cooling_schedule { // todo, fix: currently ending temp is not the same as the one used in the schedule. Float precision?
        CoolingSchedule::Linear => {
            let c = (start_temperature - end_temperature) / num_iterations as f64;
            Box::new(move |old_temp| old_temp - c)
        }
        CoolingSchedule::Geometric => {
            let c = (end_temperature / start_temperature).powf(1.0 / num_iterations as f64);
            Box::new(move |old_temp| old_temp * c)
        }
    }
}

/// Cooling schedule for simulated annealing
pub enum CoolingSchedule {
    /// New temp = old temp - c for constant c > 0
    Linear,
    /// New temp = old temp * c for constant 0 < c < 1
    Geometric,
}
