use crate::problems::problem::Problem;

pub trait Solution{
    fn solve(&self, problem: &Problem);
}