mod position;

use position::Position;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Number {
    value: u32,
    position: Position,
}

pub type Numbers = Vec<Number>;

impl Number {
    pub fn new(value: u32, row: usize, column_start: usize, column_end: usize) -> Self {
        let position = Position::new(row, column_start, column_end);
        Self { value, position }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn get_numbers(engine_schematic: &super::EngineSchematic) -> Numbers {
        let mut numbers = Vec::<Number>::new();
        for row in engine_schematic.vec2d.iter().enumerate() {
            numbers.append(&mut get_numbers_from_row(row))
        }
        numbers
    }
}

fn get_numbers_from_row(tuple: (usize, &Vec<char>)) -> Vec<Number> {
    let row = tuple.0;
    let chars = tuple.1;
    let mut new_numbers = Vec::<Number>::new();
    let mut number: Option<u32> = None;
    let mut column_start: Option<usize> = None;
    for (i, c) in chars.iter().enumerate() {
        match c {
            token if token.is_ascii_digit() => {
                let digit = token.to_digit(10).expect("We have checked before");
                number = match number {
                    Some(num) => Some(10 * num + digit),
                    None => {
                        column_start = Some(i);
                        Some(digit)
                    }
                }
            }
            _ => {
                if let Some(value) = number {
                    new_numbers.push(Number::new(
                        value,
                        row,
                        column_start.expect("We should have a value here"),
                        i - 1,
                    ));
                    number = None;
                    column_start = None
                }
            }
        }
    }
    if let Some(value) = number {
        new_numbers.push(Number::new(
            value,
            row,
            column_start.expect("We should have a value here"),
            chars.len() - 1,
        ));
    }
    new_numbers
}

#[cfg(test)]
mod test {
    use super::super::EngineSchematic;
    use super::Number;
    use crate::input::tests::TEST_DATA;
    use googletest::prelude::*;
    use rstest::{fixture, rstest};

    #[googletest::test]
    #[rstest]
    #[case(TEST_DATA, get_numbers_from_test_data())]
    #[case("23",   vec![Number::new(23, 0, 0, 1)])]
    #[case(".23",  vec![Number::new(23, 0, 1, 2)])]
    #[case(".23.", vec![Number::new(23, 0, 1, 2)])]
    #[case("23.",  vec![Number::new(23, 0, 0, 1)])]
    fn test_get_numbers(#[case] input: &str, #[case] numbers: Vec<Number>) {
        let engine_schematic = EngineSchematic::parse(input);
        expect_that!(Number::get_numbers(&engine_schematic), eq(numbers))
    }

    #[fixture]
    fn get_numbers_from_test_data() -> Vec<Number> {
        vec![
            Number::new(467, 0, 0, 2),
            Number::new(114, 0, 5, 7),
            Number::new(35, 2, 2, 3),
            Number::new(633, 2, 6, 8),
            Number::new(617, 4, 0, 2),
            Number::new(58, 5, 7, 8),
            Number::new(592, 6, 2, 4),
            Number::new(755, 7, 6, 8),
            Number::new(664, 9, 1, 3),
            Number::new(598, 9, 5, 7),
        ]
    }
}
