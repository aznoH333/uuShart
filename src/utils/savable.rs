use std::path::Path;

pub trait Savable{
    fn save_to_file(&self, path: String); 
}