use rand::Rng;

use super::neighborhood::Neighborhood;
use crate::neighborhood::stepfunction::StepFunction;
use crate::solution::Solution;


pub struct PlexJoin;

impl Neighborhood for PlexJoin {
    fn get_solution<'a>(&self, solution: &mut Solution<'a>, stepfn: &StepFunction) -> bool {
        match stepfn {
            StepFunction::FirstImprovement => unimplemented!(),
            StepFunction::BestImprovement => unimplemented!(),
            StepFunction::RandomChoice => {
                let mut rng = rand::thread_rng();
                
                let component1 = rng.gen_range(0 .. solution.connection_components.len());
                let mut component2 = rng.gen_range(0 .. solution.connection_components.len());
                while component1 == component2 {
                    component2 = rng.gen_range(0 .. solution.connection_components.len());
                }
                
                for vertex1 in solution.connection_components[component1].indices.clone() {
                    for vertex2 in solution.connection_components[component2].indices.clone() {
                        if !*solution.edges.get(vertex1, vertex2) {
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