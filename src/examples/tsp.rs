// Example for the Traveling Salesman Problem (TSP).
// Input is defined as a number of points in the plane representing cities. The output is a permutation of the cities. The cost of a solution is the sum of the distances between consecutive cities in the permutation.
// Input file is a text file where the first line is an integer the number of cities, followed by one line per city with the integer x and y coordinates separated by a space.

use crate::solution;
use solution::{InstanceReader, LocalMove, Solution};
use std::sync::Arc;

pub struct TspInstance {
    points: Vec<(f64, f64)>,
}

pub struct TspSolution {
    instance: Arc<TspInstance>,
    perm: Vec<usize>,
    cost: Option<f64>,
}

impl TspSolution {
    pub fn recompute_cost(&mut self) -> f64 {
        0.0 // todo
    }
}

impl Solution for TspSolution {
    fn get_cost(&mut self) -> f64 {
        if let Some(c) = &self.cost {
            *c
        } else {
            self.recompute_cost()
        }
    }
}

pub struct TspMove {
    
}
// todo: naive, 2opt

impl LocalMove<TspSolution> for TspMove {
    fn do_random_move(&mut self, solution: &mut TspSolution) {
        todo!()
    }
}

pub struct TspInstanceReader {}

impl InstanceReader<TspSolution> for TspInstanceReader {
    fn read_instance(&self, file_path: &str) -> TspSolution {
        todo!()
    }
}
