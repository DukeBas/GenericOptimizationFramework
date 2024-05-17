use crate::solution::{LocalMove, Solution};
use rand::{rngs::SmallRng, Rng, SeedableRng};

const STARTING_ACCEPTANCE_PROBABILITY_RANDOM: f64 = 0.5;
const STARTING_ACCEPTANCE_PROBABILITY_GREEDY: f64 = 0.2;
const ENDING_ACCEPTANCE_PROBABILITY: f64 = 10e-6;

/// How often to report the status of the algorithm
const REPORT_STATUS_EVERY_ITERATION: u32 = 1_000_000;

/// How many status checks need to be the same before early returning
const EARLY_RETURN_TIMES: u32 = 5;

const FLOAT_PRECISION: f64 = 10e-6;

// TODO add some mechanism to auto save best every so often
/// Simulated annealing algorithm, automatically determines temperature.
/// Cools after every iteration.
/// TODO: currently only is for minimization problems, should be generalized?
pub fn simulated_annealing<M, T>(
    solution: &mut T,
    num_iterations: u32,
    num_iterations_temperature_determining: u32,
    cooling_schedule: CoolingSchedule,
    greedy_start: bool,
) where
    M: LocalMove<T>,
    T: Solution,
{
    // Determine starting temperature by doing random moves and checking variance in solution cost
    let mut old_cost;
    let mut total_cost_diff = 0.0;
    let mut smallest_cost_diff = f64::INFINITY;
    for _ in 0..num_iterations_temperature_determining {
        old_cost = solution.get_cost();

        M::do_random_move(solution);

        let cost_diff = (solution.get_cost() - old_cost).abs();

        if cost_diff <= 0.0001 {
            continue; // Makes sure 0 does not count for the smallest difference
        }

        total_cost_diff += cost_diff;

        if cost_diff.abs() < smallest_cost_diff {
            smallest_cost_diff = cost_diff.abs();
        }
    }
    let avg_cost_diff = total_cost_diff / num_iterations_temperature_determining as f64;
    let starting_accepting_probability: f64 = if greedy_start {
        // Base starting acceptance on whether the initial solution is greedy or random
        STARTING_ACCEPTANCE_PROBABILITY_GREEDY
    } else {
        STARTING_ACCEPTANCE_PROBABILITY_RANDOM
    };
    let starting_temperature = -avg_cost_diff / starting_accepting_probability.ln();

    // Determine ending temperature
    let ending_accepting_probability: f64 = ENDING_ACCEPTANCE_PROBABILITY;
    let ending_temperature = -smallest_cost_diff / ending_accepting_probability.ln();

    // Get the cooling schedule
    let cooling_schedule: Box<dyn Fn(f64) -> f64> = get_cooling_schedule(
        cooling_schedule,
        starting_temperature,
        ending_temperature,
        num_iterations,
    );

    // Setup early return
    let mut early_return_counter = 0;
    let mut last_status_check_cost = solution.get_cost();

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
            }
        }

        // Update temperature
        temperature = cooling_schedule(temperature);

        // Update previous cost
        previous_cost = new_cost;

        // print cost every so often
        if it % REPORT_STATUS_EVERY_ITERATION == 0 {
            let percentage = (it as f64 / num_iterations as f64) * 100.0;
            println!(
                " {:.0}% - Best cost: {:.4} Current cost: {:.4} Temp: {:.4} ",
                percentage,
                best_solution.get_cost(),
                solution.get_cost(),                
                temperature,
            );

            // Update early return counter
            if (solution.get_cost() - last_status_check_cost).abs() < FLOAT_PRECISION {
                early_return_counter += 1;

                // Early return if the same solution is found multiple times
                if early_return_counter >= EARLY_RETURN_TIMES {
                    let percentage = (it as f64 / num_iterations as f64) * 100.0;
                    println!("Early return at iteration {} ({:.0}% done)", it, percentage);
                    break;
                }
            } else {
                early_return_counter = 0;
                last_status_check_cost = solution.get_cost();
            }
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

/// Metropolis rule for simulated annealing.
#[inline(always)]
fn metropolis_rule(cost_difference: f64, temperature: f64) -> f64 {
    (cost_difference / temperature).exp()
}

fn get_cooling_schedule(
    cooling_schedule: CoolingSchedule,
    start_temperature: f64,
    end_temperature: f64,
    num_iterations: u32,
) -> Box<dyn Fn(f64) -> f64> {
    match cooling_schedule {
        // todo, fix: currently ending temp is not the same as the one used in the schedule. Float precision?
        CoolingSchedule::Linear => {
            let c = (start_temperature - end_temperature) / num_iterations as f64;
            Box::new(move |old_temp| old_temp - c)
        }
        CoolingSchedule::Exponential => {
            let c = (end_temperature / start_temperature).powf(1.0 / num_iterations as f64);
            Box::new(move |old_temp| old_temp * c)
        }
    }
}

/// Cooling schedule for simulated annealing
#[allow(dead_code)]
pub enum CoolingSchedule {
    /// Arithmetic, new temp = old temp - c for constant c > 0
    Linear,
    /// Geometric, New temp = old temp * c for constant 0 < c < 1
    Exponential,
    // TODO: add more schedules, like constant thermodynamic https://www.fys.ku.dk/~andresen/BAhome/ownpapers/perm-annealSched.pdf adaptive https://arxiv.org/pdf/2002.06124
}
