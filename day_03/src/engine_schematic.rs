mod number;
mod parser;

use number::Number;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EngineSchematic {
    vec2d: Vec<Vec<char>>,
}

impl EngineSchematic {
    pub fn parse(input: &str) -> Self {
        let array2d = parser::input_to_2darray(input);
        Self { vec2d: array2d }
    }

    pub fn get_numbers(&self) -> Vec<u32> {
        Number::get_numbers(self)
            .iter()
            .map(|n| n.value())
            .collect()
    }
}
