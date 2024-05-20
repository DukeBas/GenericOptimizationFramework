// Example for the Traveling Salesman Problem (TSP).
// Input is defined as a number of points in the plane representing cities. The output is a permutation of the cities. The cost of a solution is the sum of the distances between consecutive cities in the permutation.
// Input file is a text file where the first line is an integer the number of cities, followed by one line per city with the integer x and y coordinates separated by a space.

use crate::solution;
use rand::{seq::SliceRandom, thread_rng};
use solution::{InstanceReader, LocalRandomMove, Solution};
use std::sync::Arc;
use std::io::Write;

pub struct TspInstance {
    dataset_name: String,
    points: Vec<(f64, f64)>,
}

#[derive(Clone)]
pub struct TspSolution {
    instance: Arc<TspInstance>,
    perm: Vec<usize>,
    cost: f64,
    // These two below exact meaning depends on the move.
    last_swap: (usize, usize),
    last_cost: f64,
}

impl TspSolution {
    pub fn recompute_cost_from_scratch(&mut self) -> f64 {
        let mut new_cost = 0.0;
        for i in 0..self.perm.len() {
            let (x1, y1) = self.instance.points[self.perm[i]];
            let (x2, y2) = self.instance.points[self.perm[(i + 1) % self.perm.len()]];
            new_cost += ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
        }
        self.cost = new_cost;
        new_cost
    }

    // /// Get the length of the edge between the i-th and (i+1)-th city in the permutation
    // fn get_edge_length_next(&self, i: usize) -> f64 {
    //     let (x1, y1) = self.instance.points[self.perm[i]];
    //     let (x2, y2) = self.instance.points[self.perm[(i + 1) % self.perm.len()]];
    //     ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    // }

    // fn get_edge_length_prev(&self, i: usize) -> f64 {
    //     let (x1, y1) = self.instance.points[self.perm[i]];
    //     let (x2, y2) = self.instance.points[self.perm[(i + self.perm.len() - 1) % self.perm.len()]];
    //     ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    // }
}

impl Solution for TspSolution {
    fn get_cost(&mut self) -> f64 {
        self.cost
    }

    fn write_solution(&self, file_location: &str) {
        // File name will be dataset name + cost + .out
        let file_path = format!("{}/{}-{:.4}.out", file_location, self.instance.dataset_name, self.cost); // todo make more robust

        let mut file = std::fs::File::create(file_path.clone()).expect("Could not save solution to file!!!");
        for city in &self.perm {
            // Get index of city in the instance
            let (x, y) = self.instance.points[*city];
            let index = self.instance.points.iter().position(|&p| p == (x, y)).unwrap();
            writeln!(file, "{}", index).expect("Could not write to file");
        }    

        println!("Solution written to {}", file_path);
    }
}

pub struct TspNaiveMove;
impl LocalRandomMove<TspSolution> for TspNaiveMove {
    fn do_random_move(solution: &mut TspSolution) {
        // Swap two random cities
        let i = rand::random::<usize>() % solution.perm.len();
        let j = rand::random::<usize>() % solution.perm.len();
        solution.perm.swap(i, j);

        // Update last swap
        solution.last_swap = (i, j);

        // Update cost by recomputing it from scratch
        // Not efficient!!!
        solution.recompute_cost_from_scratch();

        // Update last cost
        solution.last_cost = solution.cost;
    }

    fn undo_last_move(solution: &mut TspSolution) {
        // Reverse the swap
        let (i, j) = solution.last_swap;
        solution.perm.swap(i, j);

        // Update cost by recomputing it from scratch
        solution.cost = solution.last_cost;
    }
}

pub struct Tsp2OptMove; // Note: currently not _really_ 2Opt as it does not check all possible swaps
impl LocalRandomMove<TspSolution> for Tsp2OptMove {
    fn do_random_move(solution: &mut TspSolution) {
        // Reverse a random subsequence of cities
        let i = rand::random::<usize>() % solution.perm.len();
        let j = rand::random::<usize>() % solution.perm.len();

        // Make sure i < j
        let (i, j) = if i < j { (i, j) } else { (j, i) };

        // Get lengths of edges to be removed
        // let removed_i = solution.get_edge_length_next(i);
        // let removed_j = solution.get_edge_length_prev(j);

        // Do the swap
        solution.perm[i..j].reverse();

        // Get lengths of edges that were added
        // let added_i = solution.get_edge_length_next(i);
        // let added_j = solution.get_edge_length_prev(j);

        // Update last swap and cost
        solution.last_swap = (i, j);
        solution.last_cost = solution.cost;

        // Update cost
        // solution.cost += added_i + added_j - removed_i - removed_j;
        solution.cost = solution.recompute_cost_from_scratch(); // todo fix properly
    }

    fn undo_last_move(solution: &mut TspSolution) {
        // Reverse the swap
        let (i, j) = solution.last_swap;
        solution.perm[i..j].reverse();

        // Update cost
        solution.cost = solution.last_cost;
    }
}

pub struct TspInstanceReader {}
impl InstanceReader<TspSolution> for TspInstanceReader {
    fn read_instance(&self, file_path: &str, instance_name: Option<&str>) -> TspSolution {
        // TSP instance first reads the number of cities
        let contents = std::fs::read_to_string(file_path).expect("Could not read file");
        let mut lines = contents.lines();
        let num_cities = lines.next().unwrap().parse::<usize>().unwrap();

        // Then reads the coordinates of each city, where each line has its x and y as integers
        let mut points = Vec::new();
        for _ in 0..num_cities {
            let line = lines.next().unwrap();
            let mut coords = line.split_whitespace();
            let x = coords.next().unwrap().parse::<f64>().unwrap();
            let y = coords.next().unwrap().parse::<f64>().unwrap();
            points.push((x, y));
        }

        // Initialize the solution with a random permutation of the cities
        let mut perm = (0..num_cities).collect::<Vec<usize>>();
        let mut rng = thread_rng();
        perm.shuffle(&mut rng);

        // Compute the cost of the initial solution
        let mut solution = TspSolution {
            instance: Arc::new(TspInstance {
                dataset_name: instance_name.unwrap_or("unknown").to_string(),
                points,
            }),
            perm,
            cost: 0.0, // will get overriden by recompute_cost_from_scratch
            last_swap: (0, 0),
            last_cost: 0.0,
        };
        solution.recompute_cost_from_scratch();
        solution
    }
}
