use crate::engine_schematic::EngineSchematic;

pub fn solve(input: &str) -> (u32, u32) {
    let engine_schematic = EngineSchematic::parse(input);
    let result1 = engine_schematic.get_numbers().iter().sum::<u32>();
    (result1, 0)
}

#[cfg(test)]
mod tests {
    use crate::input::tests::TEST_DATA;
    use crate::input::tests::TEST_DATA_RESULT;
    use googletest::prelude::*;

    #[googletest::test]
    fn test_solve() {
        expect_that!(super::solve(TEST_DATA), eq((TEST_DATA_RESULT, 0)))
    }

    #[googletest::test]
    fn test_test_data_format() {
        expect_that!(
            TEST_DATA
                .lines()
                .next()
                .expect("We should have a first line"),
            eq("467..114..")
        );
    }
}
