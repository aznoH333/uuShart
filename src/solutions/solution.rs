
pub struct Solution{
    label: String,
    solving_steps: Vec<String>,
}

impl Solution{
    pub fn new(label: String, solving_steps: Vec<String>) -> Solution{
        return Solution{
            label: label,
            solving_steps: solving_steps,
        };
    }
}