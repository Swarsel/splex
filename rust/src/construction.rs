use crate::{solution::Solution, graph::Graph};

pub trait ConstructionHeuristic {
    fn construct(graph: &Graph) -> Solution;
}

pub struct Greedy;

impl ConstructionHeuristic for Greedy {
    fn construct(graph: &Graph) -> Solution {
        // 1. Identify connection components
        // 2. For each component: determine if better to add or remove edges
        // 3. Identify edges with minimum weight to add or remove
        let solution = Solution::default();

        

        solution
    }
}