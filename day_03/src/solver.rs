pub fn solve(input: &str) -> (u32, u32) {
    let result1 = input.len().try_into().expect("This should fit");
    (result1, 0)
}

#[cfg(test)]
mod tests {
    use googletest::prelude::*;
    use indoc::indoc;

    const TEST_DATA: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};
    const TEST_DATE_RESULT: u32 = 4361;

    #[googletest::test]
    fn test_solve() {
        expect_that!(super::solve(TEST_DATA), eq((TEST_DATE_RESULT, 0)))
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
