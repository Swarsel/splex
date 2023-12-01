use rand::Rng;

use super::neighborhood::Neighborhood;
use crate::neighborhood::stepfunction::StepFunction;
use crate::solution::Solution;

pub struct PlexDissolve;

impl Neighborhood for PlexDissolve {
    fn get_solution<'a>(&self, solution: &mut Solution<'a>, stepfn: &StepFunction) -> bool {
        match stepfn {
            StepFunction::FirstImprovement => unimplemented!(),
            StepFunction::BestImprovement => unimplemented!(),
            StepFunction::RandomChoice => {
                let mut rng = rand::thread_rng();
                
                let component = rng.gen_range(0 .. solution.connection_components.len());
                
                for vertex1 in solution.connection_components[component].indices.clone() {
                    for vertex2 in solution.connection_components[component].indices.clone() {
                        if *solution.edges.get(vertex1, vertex2) {
                            solution.flip_edge(vertex1,vertex2);
                        }
                    }
                }

                solution.recalculate_connection_components();

                true
            },
        }
    }
}