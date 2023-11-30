use crate::neighborhood::neighborhood::Neighborhood;
use crate::neighborhood::stepfunction::StepFunction;
use crate::solution::Solution;
use rand::Rng;

pub struct NFlip {
    pub n: u8,
}

impl Neighborhood for NFlip {
    fn get_solution<'a>(&self, solution: &mut Solution<'a>, stepfn: &StepFunction) -> bool {
        let mut found = false;
        let mut sol = solution.clone();
        let mut prev_flip = None;
        for _ in 0..=self.n {
            match stepfn {
                StepFunction::BestImprovement => {
                    while let Some(current_flip) = NFlip::next(&mut sol, prev_flip) {
                        if sol.is_valid() && sol.cost < solution.cost {
                            *solution = sol.clone();
                            found = true;
                        }
                        prev_flip = Some(current_flip);
                    }
                }
                StepFunction::FirstImprovement => {
                    while let Some(current_flip) = NFlip::next(&mut sol, prev_flip) {
                        if sol.is_valid() && sol.cost < solution.cost {
                            *solution = sol;
                            found = true;
                            break;
                        }
                        prev_flip = Some(current_flip);
                    }
                }
                StepFunction::RandomChoice => {
                    let size = solution.edges.len();
                    let searchlen = size * (size + 1) / 2;
                    // this is possibly crude, but the best I could come up with
                    for _ in 0..=searchlen {
                        NFlip::random(&mut sol);
                        if sol.is_valid() {
                            *solution = sol;
                            found = true;
                            break;
                        }
                    }
                }
            }

            if found == true {
                sol = solution.clone();
                found = false;
                prev_flip = None;
                continue;
            } else {
                break;
            }
        }
        found
    }
}

impl NFlip {
    pub fn new(n: u8) -> Self {
        Self { n }
    }

    fn next(solution: &mut Solution, prev: Option<(usize, usize)>) -> Option<(usize, usize)> {
        let current_flip = match prev {
            Some((prev_row, prev_col)) => {
                solution.flip_edge(prev_row, prev_col);

                if prev_col == solution.edges.len() - 1 {
                    (prev_row + 1, prev_row + 2)
                } else {
                    (prev_row, prev_col + 1)
                }
            }
            None => (0, 1),
        };

        if current_flip.0 == solution.edges.len() - 1 {
            return None;
        }

        solution.flip_edge(current_flip.0, current_flip.1);

        return Some(current_flip);
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
