mod examples;
mod heuristics;
mod solution;

use std::fs;
use std::num::NonZeroUsize;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};

use crate::heuristics::simulated_annealing::simulated_annealing;

use crate::examples::tsp::{Tsp2OptMove, TspInstanceReader, TspSolution};
use crate::solution::{InstanceReader, Solution};

/// Path to the folder containing the problem instances
const DATASET_PATH: &str = "./input/";

/// Problem instance reader to use
const INSTANCE_READER: TspInstanceReader = TspInstanceReader {};

/// Default number of iterations to run the algorithm for
const DEFAULT_NUMBER_OF_ITERATIONS: u32 = 500_000_000;

fn main() -> std::io::Result<()> {
    // Read all instances from the input folder
    let paths = fs::read_dir(DATASET_PATH)
        .unwrap()
        .map(|p| p.unwrap().path());
    let paths_vec: Vec<String> = paths.map(|p| p.to_str().unwrap().to_string()).collect();

    // Ask user which instance to run
    let instance_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an instance")
        .items(&paths_vec)
        .default(0)
        .interact()
        .unwrap();
    let instance_path = &paths_vec[instance_selection];
    let instance_name = instance_path
        .split('/')
        .last()
        .unwrap()
        .split('.')
        .next()
        .unwrap();

    // Get number of threads of the system
    let num_cpus = std::thread::available_parallelism();
    let num_cpus = num_cpus.unwrap_or(NonZeroUsize::new(1).unwrap()).get() as u32;

    // Ask for the number of threads to utilize
    let number_of_threads: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Number of threads (enter to use default)")
        .default(num_cpus)
        .interact_text()
        .unwrap();

    // Ask the user for the number of iterations
    let number_of_iterations: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Number of iterations (enter to use default)")
        .default((DEFAULT_NUMBER_OF_ITERATIONS as u32).into())
        .interact_text()
        .unwrap();

    // Always SA for now TODO: make (list of) algo(s) configurable above
    let solution = INSTANCE_READER.read_instance(&instance_path, Some(instance_name));

    // Spawn threads
    let handles: Vec<_> = (0..number_of_threads)
        .map(|i| {
            let solution = solution.clone();
            let name: String = instance_name.to_owned() + &i.to_string();
            std::thread::spawn(move || {
                infinite_loop(solution, number_of_iterations, &name);
            })
        })
        .collect();

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    } // Should never reach here

    Ok(())
}

fn infinite_loop(mut solution: TspSolution, number_of_iterations: u32, process_name: &str) -> ! {
    // Main loop, run algo until cancelled
    loop {
        simulated_annealing::<Tsp2OptMove, TspSolution>(
            &mut solution,
            number_of_iterations,
            20_000,
            crate::heuristics::simulated_annealing::CoolingSchedule::Exponential,
            false,
            process_name,
        );
        solution.write_solution("output");
    }
}

// General function to run a heuristic on a solution, should take a solution
//Todo:
// Generalize minimize/maximise
// Algorithm selection
// Early stop still saves best solution
