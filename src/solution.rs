pub trait Solution: Clone {
    /// Returns the cost of the solution. Could be recomputed from scratch or done more smartly based on previous cost.
    fn get_cost(&mut self) -> f64;
}

pub trait LocalMove<T: Solution> {
    /// Modifies the solution in place to a random neighboring solution.
    /// For efficiency, the move should update the cost function in the process instead of recomputing it from scratch.
    fn do_random_move(solution: &mut T);

    /// Undoes the last move done by do_random_move. Should reset the cost function as well if not recomputed from scratch.
    fn undo_last_move(solution: &mut T);
}

pub trait InstanceReader<T: Solution> {
    /// Reads an instance from a file. Note that an initial (random/greedy) solution should be generated as well.
    fn read_instance(&self, file_path: &str) -> T;
}
