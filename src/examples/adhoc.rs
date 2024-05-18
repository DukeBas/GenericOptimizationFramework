use crate::solution;
use rand::Rng;
use rand::{seq::SliceRandom, thread_rng};
use solution::{InstanceReader, LocalMove, Solution};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::sync::Arc;

/// Each project has a person cap and a topic (one of the proficiencies)
struct InputProject {
    person_cap: usize,
    topic: String,
}

/// Each student is a hashmap of grades and set preferences for other students (usize)
struct Student {
    grades: HashMap<String, f64>,
    preferences: HashSet<usize>,
}

/// Each mentor has a set of proficiencies
struct Mentor {
    proficiencies: HashSet<String>,
}

/// A project in the solution
struct SolutionProject {
    project: InputProject,
    students: HashSet<usize>, // indexes into instance students
    mentor: usize,            // index into instance mentors
}

pub struct AdHocInstance {
    dataset_name: String,
    projects: Vec<InputProject>,
    students: Vec<Student>,
    mentors: Vec<Mentor>,
}

#[derive(Clone)]
pub struct AdHocSolution {
    instance: Arc<AdHocInstance>,
    // perm: Vec<usize>,
    cost: f64,
    // // These two below exact meaning depends on the move.
    // last_swap: (usize, usize),
    // last_cost: f64,
}

impl AdHocSolution {
    pub fn recompute_cost_from_scratch(&mut self) -> f64 {
        //
        0.0
    }
}

impl Solution for AdHocSolution {
    fn get_cost(&mut self) -> f64 {
        self.cost
    }

    fn write_solution(&self, file_location: &str) {
        // // File name will be dataset name + cost + .out
        // let file_path = format!("{}/{}-{:.4}.out", file_location, self.instance.dataset_name, self.cost); // todo make more robust

        // let mut file = std::fs::File::create(file_path.clone()).expect("Could not save solution to file!!!");
        // for city in &self.perm {
        //     // TODO: bit of a weird way to output, indices in permutation would make more sense
        //     let (x, y) = self.instance.points[*city];
        //     writeln!(file, "{} {}", x, y).expect("Could not write to file!!!");
        // }

        // println!("Solution written to {}", file_path);
    }
}

pub struct AdHocMove;
impl LocalMove<AdHocSolution> for AdHocMove {
    fn do_random_move(solution: &mut AdHocSolution) {
        // Swap students or mentors between groups
        let mut rng = thread_rng();
        let random_number = rng.gen_range(0..solution.s + mentors);
        // rand [0, studs+mentors], if between 0 and studs, swap students, else swap mentors
    }

    fn undo_last_move(solution: &mut AdHocSolution) {
        // // Reverse the swap
        // let (i, j) = solution.last_swap;
        // solution.perm.swap(i, j);

        // // Update cost by recomputing it from scratch
        // solution.cost = solution.last_cost;
    }
}

pub struct AdHocInstanceReader {}
impl InstanceReader<AdHocSolution> for AdHocInstanceReader {
    fn read_instance(&self, file_path: &str, instance_name: Option<&str>) -> AdHocSolution {
        // todo!
        AdHocSolution {
            instance: Arc::new(AdHocInstance {
                dataset_name: instance_name.unwrap().to_string(),
                projects: vec![],
                students: vec![],
                mentors: vec![],
            }),
            cost: 0.0,
        }
    }
}
