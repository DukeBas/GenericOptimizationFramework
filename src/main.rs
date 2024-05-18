mod examples;
mod heuristics;
mod solution;

use std::fs;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use examples::adhoc::{AdHocInstanceReader, AdHocMove, AdHocSolution};

use crate::heuristics::simulated_annealing::simulated_annealing;
use crate::solution::{InstanceReader, Solution};

/// Path to the folder containing the problem instances
const DATASET_PATH: &str = "./input/";

/// Problem instance reader to use
const INSTANCE_READER: AdHocInstanceReader = AdHocInstanceReader {};

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

    // Ask the user for the number of iterations
    let number_of_iterations: u32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Number of iterations (enter to use default)")
        .default((500_000_000 as u32).into())
        .interact_text()
        .unwrap();

    // Always SA for now TODO: make (list of) algo(s) configurable above
    let mut solution = INSTANCE_READER.read_instance(&instance_path, Some(instance_name));

    // Main loop, run algo until cancelled
    loop {
        simulated_annealing::<AdHocMove, AdHocSolution>(
            &mut solution,
            number_of_iterations,
            20_000,
            crate::heuristics::simulated_annealing::CoolingSchedule::Exponential,
            false,
        );
        solution.write_solution("output");
    }
}

// General function to run a heuristic on a solution, should take a solution
//Todo:
// Generalize minimize/maximise
// Algorithm selection
// Threading
