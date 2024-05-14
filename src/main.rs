struct TspInstance {
    points: Vec<(f64, f64)>,
}

struct TspSolution {
    instance: &TspInstance,
    perm: Vec<usize>,
    cost: Option<f64>,
}

impl TspSolution {
    pub fn recompute_cost(&mut self) -> f64 {}
}

impl Solution for TspSolution {
    pub fn get_cost(&mut self) -> f64 {
        if let Some(c) = &self.cost {
            c
        } else {
            self.recompute_cost()
        }
    }
}

struct TspMove {
    i: usize,
    j: usize,
}

impl LocalMove for TspMove {
    pub fn get_random(solution: &T) -> LocalMove {}
    pub fn get_all(solution: &T) -> impl Iterator<Self> {}
    pub fn apply(&self, solution: &mut T) {}
    pub fn undo(&self, solution: &mut T) {}
}

fn main() {
    println!("Hello, world!");

    // select problem
    // select dataset

    // Select algo -> SA + move
}
