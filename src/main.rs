mod examples;
mod heuristics;
mod solution;

use crate::heuristics::simulated_annealing::simulated_annealing;

use crate::examples::tsp::{TspInstanceReader, TspNaiveMove, TspSolution};
use crate::solution::InstanceReader;

/// Path to the folder containing the problem instances
const DATASET_PATH: &str = "input/";

fn main() {
    println!("Hello, world!");

    // select dataset, todo, for now just hardcode a path
    let dataset_path = DATASET_PATH.to_string() + "tsp_test.in";
    println!("Reading dataset from {}", dataset_path);

    // Select algo -> SA + move
    let reader = TspInstanceReader {};

    // Always SA for now
    let mut solution = reader.read_instance(&dataset_path);

    simulated_annealing::<TspNaiveMove, TspSolution>(
        &mut solution,
        1.0,
        10000,
        500,
        crate::heuristics::simulated_annealing::CoolingSchedule::Geometric,
    );
}
