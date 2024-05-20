mod examples;
mod heuristics;
mod solution;

use std::fs;
use std::num::NonZeroUsize;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use heuristics::setup_simulated_annealing;

use crate::examples::tsp::{Tsp2OptMove, TspInstanceReader, TspSolution};
use crate::solution::InstanceReader;

use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

/// Path to the folder containing the problem instances
const DATASET_PATH: &str = "./input/";

/// Default number of iterations to run the algorithm for
const DEFAULT_NUMBER_OF_ITERATIONS: u32 = 500_000_000;

/// Define solution type and move. Override these for your problem!
type MoveType = Tsp2OptMove;
type SolutionType = TspSolution;

/// Problem instance reader to use
const INSTANCE_READER: TspInstanceReader = TspInstanceReader {};

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

    let heuristics = Heuristics::iter().collect::<Vec<_>>();
    let heuristic_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a heuristic")
        .items(&heuristics)
        .default(0)
        .interact()
        .unwrap();
    let heuristic = &heuristics[heuristic_choice];

    // Set up Ctrl+C handler
    let stop_signal: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    // Setup starting solution
    let solution = INSTANCE_READER.read_instance(&instance_path, Some(instance_name));

    // Run the selected heuristic
    match heuristic {
        Heuristics::SimulatedAnnealing => {
            setup_simulated_annealing::<MoveType, SolutionType>(
                instance_name,
                solution,
                stop_signal,
            );
        }
        _ => {
            println!("Heuristic not implemented yet");
        }
    }

    Ok(())
}

/// Gets the number of system threads
fn get_thread_count() -> u32 {
    let num_cpus = std::thread::available_parallelism();
    let num_cpus = num_cpus.unwrap_or(NonZeroUsize::new(1).unwrap()).get() as u32;
    num_cpus
}

#[derive(EnumIter, Display, Clone, Copy, Debug)]
enum Heuristics {
    SimulatedAnnealing,
    Tempering,
    TabuSearch,
    AntColonyOptimization,
    ParticleSwarmOptimization,
}

//Todo:
// General function to run a heuristic on a solution, should take a solution -> decompose SA with outer being the saving as well. Then reuse this for...
// tempering
// Make higher level reading api    -> x,y,z= ...
