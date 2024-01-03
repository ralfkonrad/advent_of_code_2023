#[derive(Debug)]
pub struct Draw {}

pub type Draws = Vec<Draw>;

impl Draw {
    pub fn parse(_cubes: &str) -> Draws {
        Vec::new()
    }
}
