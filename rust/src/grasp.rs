use crate::{graph::Graph, solution::Solution, vnd::VND, construction::ConstructionHeuristic};

pub struct GRASP {
    construction_heuristic: Box<dyn for<'a> ConstructionHeuristic<'a>>,
    vnd: VND,
}

impl GRASP {
    pub fn new<'a>(construction_heuristic: Box<dyn ConstructionHeuristic<'a>>, vnd: VND) -> Self {
        Self { construction_heuristic, vnd }
    }

    pub fn run<'a>(&self, graph: &'a Graph) -> Solution<'a> {
        let mut best_solution = None;

        for it in 0..3 {
            let solution = self.construction_heuristic.construct(graph, true);
            let solution_candidate = self.vnd.run(solution);

            match best_solution {
                None => best_solution = Some(solution_candidate),
                Some(ref mut best) => {
                    if solution_candidate.cost < best.cost {
                        *best = solution_candidate;
                    }
                }
            }
        }

        best_solution.expect("Should have solution after GRASP run")
    }
}
