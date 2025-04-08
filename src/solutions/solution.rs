use std::fmt::format;


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

    pub fn get_label(&self) -> &String {
        return &self.label;
    }

    pub fn get_solving_steps(&self) -> &Vec<String> {
        return &self.solving_steps;
    }

    pub fn to_string(&self) -> String {
        return format!("[{}] -> [{}]", self.label, self.solving_steps.join("|||| "));
    }
}