use crate::solution::Solution;

pub trait StoppingCriterion {
    fn is_finished(&self, iteration: usize, current_solution: &Solution) -> bool;
}

pub struct TimedStoppingCriterion {
    start_time: std::time::Instant,
    max_duration: std::time::Duration,
}

impl TimedStoppingCriterion {
    pub fn new(max_duration: std::time::Duration) -> Self {
        Self {
            start_time: std::time::Instant::now(),
            max_duration,
        }
    }
}

impl StoppingCriterion for TimedStoppingCriterion {
    fn is_finished(&self, _iteration: usize, _current_solution: &Solution) -> bool {
        self.start_time.elapsed() >= self.max_duration
    }
}