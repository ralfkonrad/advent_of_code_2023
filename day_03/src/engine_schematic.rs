mod number;
mod parser;

use number::Number;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EngineSchematic {
    vec2d: Vec<Vec<char>>,
    rows: usize,
    columns: usize,
}

impl EngineSchematic {
    pub fn get(&self, row: usize, column: usize) -> char {
        self.vec2d[row][column]
    }

    pub fn parse(input: &str) -> Self {
        let array2d = parser::input_to_2darray(input);
        let rows = array2d.len();
        let columns = array2d[0].len();
        Self {
            vec2d: array2d,
            rows,
            columns,
        }
    }

    pub fn get_numbers(&self) -> Vec<u32> {
        Number::get_numbers(self)
            .iter()
            .map(|n| n.value())
            .collect()
    }
}
