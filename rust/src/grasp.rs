use crate::{graph::Graph, solution::Solution, vnd::VND, construction::ConstructionHeuristic};

pub struct GRASP<'a> {
    construction_heuristic: Box<dyn ConstructionHeuristic<'a>>,
    vnd: VND,
}

impl<'a> GRASP<'a> {
    pub fn new(construction_heuristic: Box<dyn ConstructionHeuristic<'a>>, vnd: VND) -> Self {
        Self { construction_heuristic, vnd }
    }

    pub fn run(&self, graph: &'a Graph) -> Solution<'a> {
        let mut best_solution = None;

        for _ in 0..1 {
            let mut solution = self.construction_heuristic.construct(graph);

            solution.recalculate_connection_components();
            solution.recalculate_degrees();

            assert!(solution.is_valid());

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
