use crate::neighborhood::neighborhood::Neighborhood;
use crate::neighborhood::stepfunction::StepFunction;
use crate::solution::Solution;

pub struct VND {
    neighborhoods: Vec<(Box<dyn Neighborhood>, StepFunction)>,
}

impl VND {
    pub fn new(
        neighborhoods: Vec<(Box<dyn Neighborhood>, StepFunction)>,
    ) -> Self {
        Self {
            neighborhoods,
        }
    }

    pub fn run<'a>(&self, mut solution: Solution<'a>) -> Solution<'a> {
        let mut k = 0;

        while k < self.neighborhoods.len() {
            let (neighborhood, step_function) = &self.neighborhoods[k];

            if neighborhood.get_solution(&mut solution, step_function) {
                k = 0;
            } else {
                k += 1;
            }
        }

        solution
    }
}
