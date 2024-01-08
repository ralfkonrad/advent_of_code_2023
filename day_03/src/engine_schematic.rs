use ndarray::Array2;

mod number;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EngineSchematic {
    array2d: Array2<char>,
}

impl EngineSchematic {
    pub fn parse(input: &str) -> Self {
        let array2d = super::input_to_2darray::input_to_2darray(input);
        Self { array2d }
    }

    pub fn get_numbers(&self) -> Vec<u32> {
        number::Number::get_numbers(self)
            .iter()
            .map(|n| n.value)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::EngineSchematic;
    use crate::input::tests::TEST_DATA;
    use googletest::prelude::*;
    use rstest::{fixture, rstest};

    #[googletest::test]
    #[rstest]
    #[case(TEST_DATA, get_numbers_from_test_data())]
    fn test_numbers(#[case] input: &str, #[case] numbers: Vec<u32>) {
        let engine_schematic = EngineSchematic::parse(input);
        expect_that!(engine_schematic.get_numbers(), eq(numbers))
    }

    #[fixture]
    fn get_numbers_from_test_data() -> Vec<u32> {
        vec![467, 114, 35, 633, 617, 58, 592, 755, 664, 598]
    }
}
