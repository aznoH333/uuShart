pub trait ProblemSource{
    fn get_next_problem(&mut self) -> Problem;
    fn is_complete() -> bool;
}