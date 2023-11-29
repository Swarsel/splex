use crate::graph::Graph;
use crate::neighborhood::neighborhood::Neighborhood;
use crate::neighborhood::stepfunction::StepFunction;
use crate::solution::Solution;
use crate::ConstructionHeuristic;

pub struct VND {
    construction_heuristic: Box<dyn for<'a> ConstructionHeuristic<'a>>,
    neighborhoods: Vec<(Box<dyn Neighborhood>, StepFunction)>,
}

impl VND {
    pub fn new(
        construction_heuristic: Box<dyn for<'a> ConstructionHeuristic<'a>>,
        neighborhoods: Vec<(Box<dyn Neighborhood>, StepFunction)>,
    ) -> Self {
        Self {
            construction_heuristic,
            neighborhoods,
        }
    }

    pub fn run<'a>(&self, graph: &'a Graph, random: bool) -> Solution<'a> {
        let mut solution = self.construction_heuristic.construct(graph, random);

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
