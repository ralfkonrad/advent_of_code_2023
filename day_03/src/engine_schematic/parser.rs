pub fn input_to_2darray(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[cfg(test)]
mod tests {
    use super::input_to_2darray;
    use crate::input::tests::TEST_DATA;
    use googletest::prelude::*;

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

    #[test]
    fn test_input_to_2darray() {
        let input = input_to_2darray(TEST_DATA);
        assert_eq!(input.len(), 10);
        assert_eq!(input[0].len(), 10);
        assert_eq!(input[0][0], '4');
        assert_eq!(input[1][3], '*');
        assert_eq!(input[9][9], '.')
    }
}
