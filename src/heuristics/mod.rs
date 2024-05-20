use std::{
    ops::ControlFlow,
    sync::{atomic::AtomicBool, Arc},
};

use crate::solution::Solution;

pub mod simulated_annealing;

pub type StopSignal = Arc<AtomicBool>;

/// How many status checks need to be the same before early returning
const EARLY_RETURN_TIMES: u32 = 5;

/// Precision for floating point numbers
const FLOAT_PRECISION: f64 = 10e-6;

/// Function to handle early return in heuristics
fn check_early_return<T>(
    stop_signal: &StopSignal,
    process_name: &str,
    solution: &mut T,
    last_status_check_cost: &mut f64,
    early_return_counter: &mut u32,
    current_iteration: u32,
    num_iterations: u32,
) -> ControlFlow<()>
where
    T: Solution,
{
    // Check for stop signal
    if stop_signal.load(std::sync::atomic::Ordering::Relaxed) {
        println!("{} - Stopping early", process_name);
        return ControlFlow::Break(());
    }

    // Update early return counter
    if (solution.get_cost() - *last_status_check_cost).abs() < FLOAT_PRECISION {
        *early_return_counter += 1;

        // Early return if the same solution is found multiple times
        if *early_return_counter >= EARLY_RETURN_TIMES {
            let percentage = (current_iteration as f64 / num_iterations as f64) * 100.0;
            println!(
                "{} - Early return at iteration {} ({:.0}% done)",
                process_name, current_iteration, percentage
            );
            return ControlFlow::Break(());
        }
    } else {
        *early_return_counter = 0;
        *last_status_check_cost = solution.get_cost();
    }
    ControlFlow::Continue(())
}
