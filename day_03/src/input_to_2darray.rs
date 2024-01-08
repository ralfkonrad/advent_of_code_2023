use ndarray::Array2;

pub fn input_to_2darray(input: &str) -> Array2<char> {
    let vector = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut arr = Array2::<char>::default((vector.len(), vector[0].len()));
    for (i, inner) in vector.iter().enumerate() {
        for (j, c) in inner.iter().enumerate() {
            arr[[i, j]] = *c
        }
    }
    arr
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
        let array = input_to_2darray(TEST_DATA);
        assert_eq!(array.dim(), (10, 10));
        assert_eq!(array[[0, 0]], '4');
        assert_eq!(array[[1, 3]], '*');
        assert_eq!(array[[9, 9]], '.')
    }
}
