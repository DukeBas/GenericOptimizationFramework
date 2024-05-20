use core::num;

use rand::rngs::SmallRng;

use crate::{
    heuristics::simulated_annealing::{
        determine_start_and_end_temp, get_cooling_schedule, sa_core,
    },
    solution::{LocalRandomMove, Solution},
};

use super::{simulated_annealing::CoolingSchedule, StopSignal};

/// Tempering is a specialisation of simulated annealing that runs multiple annealing processes in parallel.
/// The processes are run with a set temperature, without cooling. Solutions are periodically exchanged between processes.
/// Initialises random solutions and runs the tempering process. Given cooling schedule is used to distribute the temperatures.
pub fn tempering<M, T>(
    solution: &mut T,
    num_threads: u32,
    num_iterations_temperature_determining: u32,
    cooling_setup: CoolingSchedule,
    process_name: &str,
    stop_signal: StopSignal,
) where
    M: LocalRandomMove<T>,
    T: Solution,
{
    // Check if we have at least 2 threads
    if num_threads < 2 {
        panic!("Tempering requires at least 2 threads to run");
    }

    // Determine temperature of highest and lowest temperature threads
    let (starting_temperature, ending_temperature) = determine_start_and_end_temp::<M, T>(
        num_iterations_temperature_determining,
        solution,
        false,
    );

    // Get the cooling schedule to use to distribute the temperatures
    let cooling_schedule: Box<dyn Fn(f64) -> f64> = get_cooling_schedule(
        cooling_setup,
        starting_temperature,
        ending_temperature,
        num_threads,
    );

    // Setup temperatures
    let mut temperatures: Vec<f64> = vec![starting_temperature];
    for i in 1..num_threads {
        temperatures.push(cooling_schedule(temperatures[(i - 1) as usize]));
    }

    // Setup early return
    let mut early_return_counter = 0;
    let mut last_status_check_cost = solution.get_cost();

    // Print some info
    println!(
        "{} - Running tempering on {} threads, temperatures {:.3?}",
        process_name, num_threads, temperatures
    );

    // todo
    // let mut temperature = starting_temperature;
    // let mut previous_cost = solution.get_cost();
    // let mut small_rng = SmallRng::from_entropy(); // StdRng is about 4x slower when tested
    // let mut best_solution = solution.clone();
    // for it in 0..num_iterations {
    //     sa_core::<M, T>(solution, &mut previous_cost, &mut small_rng, temperature);

    //     // Update temperature
    //     temperature = cooling_schedule(temperature);

    //     // print cost every so often, check for early return
    //     if it % REPORT_STATUS_EVERY_ITERATION == 0 {
    //         let percentage = (it as f64 / num_iterations as f64) * 100.0;
    //         println!(
    //             " {} - {:.0}% - Best cost: {:.4} Current cost: {:.4} Temp: {:.4} ",
    //             process_name,
    //             percentage,
    //             best_solution.get_cost(),
    //             solution.get_cost(),
    //             temperature,
    //         );

    //         if let ControlFlow::Break(_) = check_early_return(
    //             &stop_signal,
    //             process_name,
    //             solution,
    //             &mut last_status_check_cost,
    //             &mut early_return_counter,
    //             it,
    //             num_iterations,
    //         ) {
    //             break;
    //         }
    //     }

    //     // Update best solution
    //     if solution.get_cost() < best_solution.get_cost() {
    //         best_solution = solution.clone();
    //     }
    // }

    // // Set the best solution
    // *solution = best_solution;

    // Print final cost
    println!("{} - Final cost: {}", process_name, solution.get_cost());
}
