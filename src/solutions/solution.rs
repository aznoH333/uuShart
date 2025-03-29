use crate::problems::problem_type::ProblemType;


pub struct Solution{
    label: String,
    problem_type: ProblemType,
    solve_steps: Vec<String>,
}

impl Solution{
    pub fn new(label: &String, problem_type: ProblemType) -> Solution{
        return Solution { label: label.to_owned(), problem_type: problem_type, solve_steps: Vec::new() }
    }

    pub fn push_solution_step(&mut self, step: &String){
        self.solve_steps.push(step.to_owned());
    }

    pub fn to_string(&self) -> String{
        return format!("label: {} solution: {}", self.label, self.solve_steps.join(", "));
    }
}