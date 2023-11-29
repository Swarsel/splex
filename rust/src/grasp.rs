use crate::{graph::Graph, solution::Solution, vnd::VND};

pub struct GRASP {
    vnd: VND,
}

impl GRASP {
    pub fn new(vnd: VND) -> Self {
        Self { vnd }
    }

    pub fn run<'a>(&self, graph: &'a Graph) -> Solution<'a> {
        let mut best_solution = Solution::new(graph);

        for it in 0..1 {
            // todo: add randomized greedy heuristic to construction
            let solution_candidate = self.vnd.run(&graph);
            if it == 0 {
                best_solution = solution_candidate;
            } else if solution_candidate.cost < best_solution.cost {
                best_solution = solution_candidate;
            }
        }
        best_solution
    }
}
