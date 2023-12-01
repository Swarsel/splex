use crate::{vnd::VND, neighborhood::{neighborhood::Neighborhood, stepfunction::StepFunction}, solution::Solution};


struct GVNS {
    vnd: VND,
    shaking_neighborhoods: Vec<Box<dyn Neighborhood>>,
}

impl GVNS {
    pub fn new(vnd: VND, shaking_neighborhoods: Vec<Box<dyn Neighborhood>>) -> Self {
        Self {
            vnd,
            shaking_neighborhoods,
        }
    }

    pub fn run<'a>(&self, mut solution: Solution<'a>) -> Solution<'a> {
        solution = self.vnd.run(solution);

        let mut best = solution.clone();
        
        let mut k = 0;
        while k < self.shaking_neighborhoods.len() {
            let shaking_neighborhood = &self.shaking_neighborhoods[k];
            
            shaking_neighborhood.get_solution(&mut solution, &StepFunction::RandomChoice);
            solution = self.vnd.run(solution);

            if solution.cost < best.cost {
                best = solution.clone();
                k = 0;
            } else {
                k += 1;
            }
        }

        solution
    }
}
