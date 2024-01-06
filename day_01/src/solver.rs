pub fn solve(input: &str) -> (u32, u32) {
    let result1 = input.lines().map(solve_line).sum();
    let result2 = input
        .lines()
        .map(replace_digit_line)
        .map(|line| solve_line(line.as_str()))
        .sum();

    (result1, result2)
}

fn solve_line(line: &str) -> u32 {
    let numerics: Vec<_> = line
        .chars()
        .filter(|c| c.is_numeric())
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    let first = numerics.first();
    let last = numerics.last();

    match first {
        Some(f) => f * 10 + last.expect("If there is a first, there must be a last"),
        None => 0,
    }
}

fn replace_digit_line(line: &str) -> String {
    /* This is a bit odd as 'oneight' counts as '18'.
     * With these replacements we get 'one1oneight8eight'
     * which is somehow the desired result*/
    line.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
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
        expect_that!(solve_line(line), eq(expected));
    }

    #[googletest::test]
    #[test]
    fn test_input() {
        expect_that!(solve(DATA), eq((54390, 54277)))
    }
}
