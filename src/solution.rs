pub trait Solution {
    /// Returns the cost of the solution. Could be recomputed from scratch or done more smartly based on previous cost.
    fn get_cost(&mut self) -> f64;
}

pub trait LocalMove<T: Solution> {
    fn do_random_move(solution: &mut T);

    // todo: // fn get_all(solution: &T) -> impl Iterator<Self>;
    // fn apply(&self, solution: &mut T);
    // fn undo(&self, solution: &mut T);
}

pub trait InstanceReader<T : Solution> {
    /// Reads an instance from a file
    fn read_instance(&self, file_path: &str) -> T;
}
