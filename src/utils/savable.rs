pub trait Savable{
    fn load_from_file(path: Path) -> Result<Savable>;
    fn save_to_file(&self, path: Path) -> Result; 
}