#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepFunction {
    FirstImprovement,
    BestImprovement,
    RandomChoice,
    Skip,
}
