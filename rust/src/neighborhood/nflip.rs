use crate::neighborhood::neighborhood::Neighborhood;
use crate::neighborhood::stepfunction::StepFunction;
use crate::solution::Solution;
use rand::Rng;

use itertools::Itertools;

const MAX_COMP_SIZE: usize = 60;

/// Looks at all neighbors with N flips inside a single splex
pub struct NFlip<const N: usize>;

impl<const N: usize> Neighborhood for NFlip<{ N }> {
    fn get_solution<'a>(&self, solution: &mut Solution<'a>, stepfn: &StepFunction) -> bool {
        let mut found = false;
        let mut sol = solution.clone();
        let mut prev_step_data: Option<StepData<{ N }>> = None;

        match stepfn {
            StepFunction::BestImprovement => {
                while let Some(current_step_data) = NFlip::next(&mut sol, prev_step_data) {
                    if sol.is_valid() && sol.cost < solution.cost {
                        *solution = sol.clone();
                        found = true;
                    }
                    prev_step_data = Some(current_step_data);
                }
            }
            StepFunction::FirstImprovement => {
                while let Some(current_step_data) = NFlip::next(&mut sol, prev_step_data) {
                    if sol.is_valid() && sol.cost < solution.cost {
                        *solution = sol;
                        found = true;
                        break;
                    }
                    prev_step_data = Some(current_step_data);
                }
            }
            StepFunction::RandomChoice => {
                unimplemented!()
                // let size = solution.edges.len();
                // let searchlen = size * (size + 1) / 2;
                // // this is possibly crude, but the best I could come up with
                // for _ in 0..=searchlen {
                //     NFlip::random(&mut sol);
                //     if sol.is_valid() {
                //         *solution = sol;
                //         found = true;
                //         break;
                //     }
                // }
            }
        }

        found
    }
}

struct StepData<const N: usize> {
    prev_flips: Option<Vec<Vec<usize>>>,
    combinations: itertools::Combinations<itertools::Combinations<std::vec::IntoIter<usize>>>,
    component_index: usize,
}

impl<const N: usize> NFlip<{ N }> {
    fn next(solution: &mut Solution, prev: Option<StepData<{ N }>>) -> Option<StepData<{ N }>> {
        let mut step_data = match prev {
            None => {
                let mut component_index = 0;

                while !(&N..&MAX_COMP_SIZE).contains(&&solution.connection_components[component_index]
                    .indices
                    .len())
                {
                    component_index += 1;

                    if component_index == solution.connection_components.len() {
                        return None;
                    }
                }


                let comp = &solution.connection_components[component_index].indices;
                let combinations = comp
                    .clone()
                    .into_iter()
                    .combinations(2)
                    .combinations(N);

                StepData {
                    prev_flips: None,
                    combinations,
                    component_index: 0,
                }
            }
            Some(prev) => {
                if let Some(ref prev_flips) = prev.prev_flips {
                    for edge in prev_flips.iter() {
                        solution.flip_edge(edge[0], edge[1]);
                    }
                }

                prev
            }
        };

        loop {
            match step_data.combinations.next() {
                Some(current_flip) => {
                    for edge in current_flip.iter() {
                        solution.flip_edge(edge[0], edge[1]);
                    }

                    step_data.prev_flips = Some(current_flip);

                    return Some(step_data);
                }
                None => {
                    step_data.component_index += 1;

                    if step_data.component_index == solution.connection_components.len() {
                        return None;
                    }

                    while !(&N..&MAX_COMP_SIZE).contains(&&solution.connection_components[step_data.component_index]
                        .indices
                        .len())
                    {
                        step_data.component_index += 1;

                        if step_data.component_index == solution.connection_components.len() {
                            return None;
                        }
                    }

                    let comp = &solution.connection_components[step_data.component_index].indices;
                    let combinations = comp
                        .clone()
                        .into_iter()
                        .combinations(2)
                        .combinations(N);

                    step_data.combinations = combinations;
                }
            }
        }
    }

    fn random(solution: &mut Solution) -> (usize, usize) {
        let size = solution.edges.len() - 1;
        let row = rand::thread_rng().gen_range(0..size);
        // adjust to prevent landing in col == row
        let mut col = size; // in case row rolled size - 1, there is only one option for col
        if row != size - 1 {
            col = 1 + row + rand::thread_rng().gen_range(0..size - 1 - row);
        }
        let current_flip = (row, col);

        solution.flip_edge(current_flip.0, current_flip.1);

        current_flip
    }
}
