use std::ops::Range;

use super::neighborhood::Neighborhood;
use crate::neighborhood::stepfunction::StepFunction;
use crate::solution::Solution;

use itertools::{Itertools, Product};
use rand::Rng;

/// Moves a single vertex to another connected component
pub struct MoveVertex;

struct StepData {
    plan: Product<Range<usize>, Range<usize>>,
    prev_edits: Vec<(usize, usize)>,
}

impl Neighborhood for MoveVertex {
    fn get_solution<'a>(&self, solution: &mut Solution<'a>, stepfn: &StepFunction) -> bool {
        let mut found = false;
        let mut sol = solution.clone();
        let mut prev_step_data: Option<StepData> = None;

        match stepfn {
            StepFunction::BestImprovement => {
                while let Some(current_step_data) = MoveVertex::next(&mut sol, prev_step_data) {
                    if sol.is_valid() && sol.cost < solution.cost {
                        *solution = sol.clone();
                        found = true;
                    }
                    prev_step_data = Some(current_step_data);
                }
            }
            StepFunction::FirstImprovement => {
                while let Some(current_step_data) = MoveVertex::next(&mut sol, prev_step_data) {
                    if sol.is_valid() && sol.cost < solution.cost {
                        *solution = sol;
                        found = true;
                        break;
                    }
                    prev_step_data = Some(current_step_data);
                }
            }
            StepFunction::RandomChoice => {
                let mut rng = rand::thread_rng();

                let vertex = rng.gen_range(0..solution.edges.len());
                let mut component = rng.gen_range(0..solution.connection_components.len());
                if solution.connection_components[component]
                    .indices
                    .contains(&vertex)
                {
                    component = (component + 1) % solution.connection_components.len();
                }

                MoveVertex::move_vertex(solution, vertex, component);
                if sol.is_valid() {
                    found = true;
                }
            }
        }

        solution.recalculate_connection_components();

        found
    }
}

impl MoveVertex {
    fn next<'a>(solution: &mut Solution<'a>, prev_step_data: Option<StepData>) -> Option<StepData> {
        let mut step_data = if let Some(step_data) = prev_step_data {
            // undo previous edits
            for (i, j) in step_data.prev_edits.iter() {
                solution.flip_edge(*i, *j);
            }

            step_data
        } else {
            // create edit plan
            // (vertex, component) pairs representing all possible moves
            let plan = (0usize..solution.edges.len())
                .cartesian_product(0usize..solution.connection_components.len());

            StepData {
                plan,
                prev_edits: Vec::new(),
            }
        };

        loop {
            match step_data.plan.next() {
                Some((vertex, component)) => {
                    // apply edit
                    if solution.connection_components[component]
                        .indices
                        .contains(&vertex)
                    {
                        continue;
                    }

                    let edits = MoveVertex::move_vertex(solution, vertex, component);

                    step_data.prev_edits = edits;

                    return Some(step_data);
                }
                None => {
                    return None;
                }
            }
        }
    }

    fn move_vertex<'a>(
        solution: &mut Solution<'a>,
        vertex: usize,
        component: usize,
    ) -> Vec<(usize, usize)> {
        let mut edits = Vec::new();

        for i in 0..solution.edges.len() {
            if *solution.edges.get(vertex, i) {
                edits.push((vertex, i));
                solution.flip_edge(vertex, i);
            }
        }

        // determine number of edges needed
        let needed_edges = solution.connection_components[component].indices.len() as isize
            - solution.graph.s as isize;

        if needed_edges <= 0 {
            return edits;
        }

        let needed_edges = needed_edges as usize;

        let mut edges_added = 0;

        let mut new_edges = Vec::new();

        // add all edges which are in the original graph
        for i in solution.connection_components[component]
            .indices
            .clone()
            .iter()
        {
            if *solution.graph.initial.get(vertex, *i) {
                solution.flip_edge(vertex, *i);
                edits.push((vertex, *i));
                edges_added += 1;
            } else {
                new_edges.push((vertex, *i));
            }
        }

        // sort all other edges by their cost
        new_edges.sort_by(|a, b| {
            let a_cost = solution.graph.weights.get(a.0, a.1);
            let b_cost = solution.graph.weights.get(b.0, b.1);

            a_cost.partial_cmp(&b_cost).unwrap()
        });

        if edges_added >= needed_edges {
            return edits;
        }

        // add edges until the component is connected
        new_edges
            .iter()
            .take(needed_edges - edges_added)
            .for_each(|(i, j)| {
                solution.flip_edge(*i, *j);
                edits.push((*i, *j));
            });

        edits
    }
}
