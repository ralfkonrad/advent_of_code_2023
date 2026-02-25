pub fn solve(input: &str) -> (u32, u32) {
    let result1 = input.lines().map(|line| solve_line(line, false)).sum();
    let result2 = input.lines().map(|line| solve_line(line, true)).sum();

    (result1, result2)
}

const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Single-pass scanner that finds first and last digits in a line.
/// When `include_words` is true, also matches spelled-out digit words ("one".."nine").
/// Handles overlapping words like "oneight" → first=1, last=8.
fn solve_line(line: &str, include_words: bool) -> u32 {
    let bytes = line.as_bytes();
    let mut first: Option<u32> = None;
    let mut last: u32 = 0;

    for i in 0..bytes.len() {
        let digit = if bytes[i].is_ascii_digit() {
            Some((bytes[i] - b'0') as u32)
        } else if include_words {
            DIGIT_WORDS
                .iter()
                .enumerate()
                .find(|(_, word)| bytes[i..].starts_with(word.as_bytes()))
                .map(|(idx, _)| idx as u32 + 1)
        } else {
            None
        };

        if let Some(d) = digit {
            if first.is_none() {
                first = Some(d);
            }
            last = d;
        }
    }

    match first {
        Some(f) => f * 10 + last,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::input::DATA;
    use crate::solver::*;
    use googletest::prelude::*;
    use rstest::rstest;

    #[googletest::test]
    #[rstest]
    #[case("", 0)]
    #[case("123", 13)]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn test(#[case] line: &str, #[case] expected: u32) {
        expect_that!(solve_line(line, false), eq(expected));
    }

    #[googletest::test]
    fn test_input() {
        expect_that!(solve(DATA), eq((54390, 54277)))
    }
}
