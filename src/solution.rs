
pub trait Solution {
    pub fn get_cost(&mut self) -> f64;
}

pub trait LocalMove<T: Solution> {
    pub fn get_random(solution: &T) -> LocalMove;
    pub fn get_all(solution: &T) -> impl Iterator<Self>;
    pub fn apply(&self, solution: &mut T);
    pub fn undo(&self, solution: &mut T);
}
