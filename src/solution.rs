pub trait Solution: Clone {
    /// Returns the cost of the solution. Could be recomputed from scratch or done more smartly based on previous cost.
    fn get_cost(&mut self) -> f64;

    /// Writes solution to a file. Useful for submitting solutions to online judges and to read back later.
    fn write_solution(&self, file_path: &str);
    
    // /// Reads a solution from a file. Overrides previously saved solution. Instance should have been read beforehand.
    // /// Note that write and read should be compatible, i.e. the same format, and idempotent, i.e. read(write(x)) == x.
    // fn read_override_solution(file_path: &str) -> Self;
}

pub trait LocalRandomMove<T: Solution> {
    /// Modifies the solution in place to a random neighboring solution.
    /// For efficiency, the move should update the cost function in the process instead of recomputing it from scratch.
    /// Needs to make sure all solutions are reachable by a sequence of moves.
    fn do_random_move(solution: &mut T);

    /// Undoes the last move done by do_random_move. Should reset the cost function as well if not recomputed from scratch.
    fn undo_last_move(solution: &mut T);
}

pub trait MoveGenerator<T: Solution, S> {
    /// Generates a list of all possible moves from the current solution.
    fn generate_moves(solution: &T) -> Vec<S>;
}

pub trait InstanceReader<T: Solution> {
    /// Reads an instance from a file. Note that an initial (random/greedy) solution should be generated as well.
    fn read_instance(&self, file_path: &str, instance_name: Option<&str>) -> T;
}
