use super::neighborhood::Neighborhood;
use crate::neighborhood::movevertex::MoveVertex;
use crate::neighborhood::stepfunction::StepFunction;
use crate::solution::Solution;

/// Moves N vertices to another connected component
/// Only used for shaking
pub struct MoveNVertices<const N: usize>;

impl<const N: usize> Neighborhood for MoveNVertices<N> {
    fn get_solution<'a>(&self, solution: &mut Solution<'a>, stepfn: &StepFunction) -> bool {
        match stepfn {
            StepFunction::FirstImprovement => unimplemented!(),
            StepFunction::BestImprovement => unimplemented!(),
            StepFunction::RandomChoice => {
                let move_vertex = MoveVertex;

                for _ in 0..N {
                    move_vertex.get_solution(solution, &StepFunction::RandomChoice);
                }

                true
            },
        }
    }
}