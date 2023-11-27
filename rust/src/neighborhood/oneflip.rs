use crate::neighborhood::neighborhood::Neighborhood;
use crate::solution::Solution;
use crate::neighborhood::stepfunction::StepFunction;

pub struct OneFlip;

impl Neighborhood for OneFlip {
    fn get_solution<'a>(&self, solution: &mut Solution<'a>, stepfn: &StepFunction) -> bool {
        let mut sol = solution.clone();
        let mut prev_flip = None;
        let mut found = false;

        match stepfn {
            StepFunction::BestImprovement => {
                while let Some(current_flip) = OneFlip::next(&mut sol, prev_flip) {
                    if sol.is_valid() && sol.cost < solution.cost {
                        *solution = sol.clone();
                        found = true;
                    }
                    prev_flip = Some(current_flip);
                }

                found
            }
            StepFunction::FirstImprovement => {
                while let Some(current_flip) = OneFlip::next(&mut sol, prev_flip) {
                    if sol.is_valid() && sol.cost < solution.cost {
                        *solution = sol;
                        found = true;
                        break;
                    }
                    prev_flip = Some(current_flip);
                }

                found
            }
        }
    }
}

impl OneFlip {
    fn next (solution: &mut Solution, prev: Option<(usize, usize)>) -> Option<(usize, usize)> {
        let current_flip = match prev {
            Some((prev_row, prev_col)) => {
                solution.flip_edge(prev_row, prev_col);

                if prev_col == solution.edges.len() - 1 {
                    (prev_row + 1, prev_row + 2)
                } else {
                    (prev_row, prev_col + 1)
                }
            }
            None => {
                (0, 1)
            }
        };

        if current_flip.0 == solution.edges.len() - 1 {
            return None;
        }

        solution.flip_edge(current_flip.0, current_flip.1);

        return Some(current_flip);
    }
}
