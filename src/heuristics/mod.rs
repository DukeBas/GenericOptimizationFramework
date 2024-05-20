use std::{
    ops::ControlFlow,
    sync::{atomic::AtomicBool, Arc},
};

use dialoguer::{theme::ColorfulTheme, Input};

use crate::{
    get_thread_count,
    solution::{LocalRandomMove, Solution},
};

use crate::heuristics::tempering::tempering;
use simulated_annealing::simulated_annealing;

pub mod simulated_annealing;
pub mod tempering;

/// Stop signal for all threads
pub type StopSignal = Arc<AtomicBool>;

/// How many status checks need to be the same before early returning
const EARLY_RETURN_TIMES: u32 = 5;

/// Precision for floating point numbers
const FLOAT_PRECISION: f64 = 10e-6;

/// Default number of iterations to run iterative algorithms for
const DEFAULT_NUMBER_OF_ITERATIONS: u32 = 500_000_000;

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

pub fn setup_simulated_annealing<M: LocalRandomMove<T>, T: Solution + 'static>(
    instance_name: &str,
    solution: T,
    stop_signal: StopSignal,
) {
    // Get number of threads of the system
    let num_cpus = get_thread_count();

    // Ask for the number of threads to utilize
    let number_of_threads = ask_user_num_threads(num_cpus, None);

    // Ask the user for the number of iterations
    let number_of_iterations = ask_user_num_iterations();

    // necessary for borrowing in closure
    let stop_signal_clone = stop_signal.clone();

    ctrlc::set_handler(move || {
        println!("Received stop signal, stopping all threads");
        stop_signal_clone.store(true, std::sync::atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Spawn threads
    let handles: Vec<_> = (0..number_of_threads)
        .map(|i| {
            let solution = solution.clone();
            let name: String = instance_name.to_owned() + &i.to_string();
            let stop_signal = stop_signal.clone();
            std::thread::spawn(move || {
                infinite_loop::<M, T>(solution, number_of_iterations, &name, stop_signal);
            })
        })
        .collect();

    // Print controls
    println!("Press Ctrl+C to stop the program, all threads will stop and save their best solution to output/");

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    // Should only get here when Ctrl+C is pressed

    println!("All threads stopped, exiting.");
}

pub fn setup_tempering<M: LocalRandomMove<T>, T: Solution + 'static>(
    instance_name: &str,
    solution: T,
    stop_signal: StopSignal,
) {
    // Get number of threads of the system
    let num_cpus = get_thread_count();

    // Ask for the number of threads to utilize
    let number_of_threads = ask_user_num_threads(num_cpus, Some("Number of threads (enter to use default). Tempering requires at least 2 threads but only makes sense with more."));

    // necessary for borrowing in closure
    let stop_signal_clone = stop_signal.clone();

    ctrlc::set_handler(move || {
        println!("Received stop signal, stopping all threads");
        stop_signal_clone.store(true, std::sync::atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Print controls
    println!("Press Ctrl+C to stop the program, all threads will stop and the best solution will be saved to output/");

    // Do tempering
    let mut solution = solution.clone();
    tempering::<M, T>(
        &mut solution,
        number_of_threads,
        50_000,
        crate::heuristics::simulated_annealing::CoolingSchedule::Linear,
        instance_name,
        stop_signal.clone(),
    );

    println!("All threads stopped, exiting.");
}

/// TODO: improve this, generalize to just take a closure?
fn infinite_loop<M: LocalRandomMove<T>, T: Solution>(
    mut solution: T,
    number_of_iterations: u32,
    process_name: &str,
    stop_signal: StopSignal,
) {
    // Main loop, run algo until cancelled
    loop {
        simulated_annealing::<M, T>(
            &mut solution,
            number_of_iterations,
            20_000,
            crate::heuristics::simulated_annealing::CoolingSchedule::Exponential,
            false,
            process_name,
            stop_signal.clone(),
        );
        solution.write_solution("output");

        // Check stop signal
        if stop_signal.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }
    }
}

fn ask_user_num_iterations() -> u32 {
    let number_of_iterations: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Number of iterations (enter to use default)")
        .default((DEFAULT_NUMBER_OF_ITERATIONS as u32).into())
        .interact_text()
        .unwrap();
    number_of_iterations
}

fn ask_user_num_threads(default_num_cpus: u32, prompt: Option<&str>) -> u32 {
    let prompt = prompt.unwrap_or("Number of threads (enter to use default)");
    let number_of_threads: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default_num_cpus)
        .interact_text()
        .unwrap();
    number_of_threads
}
