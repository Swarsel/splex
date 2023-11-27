use crate::solution::Solution;

use super::stepfunction::StepFunction;

pub trait Neighborhood {
    fn get_solution<'a>(&self, solution: &mut Solution<'a>, stepfn: &StepFunction) -> bool;
}
