use super::Draw;
use super::Draws;
use regex::Regex;

pub fn parse(cubes: &str) -> Draws {
    cubes.split(';').map(parse_draw).collect()
}

fn parse_draw(cube: &str) -> Draw {
    let trimmed = cube.trim();
    let blue = capture(trimmed, "blue");
    let red = capture(trimmed, "red");
    let green = capture(trimmed, "green");
    Draw { blue, red, green }
}

fn capture(trimmed: &str, color: &str) -> u32 {
    let regex = Regex::new(&format!("(?<number>\\d+) {color}"))
        .expect("This should be a valid regular expression");
    let captures = regex.captures(trimmed);
    match captures {
        Some(capture) => capture["number"]
            .parse::<u32>()
            .expect("We should catch 'number', otherwise it would be None"),
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::game::draw::{Draw, Draws};
    use googletest::prelude::*;
    use rstest::{fixture, rstest};

    const DRAWS_STR: &str = "5 blue, 7 red; 2 green, 1 blue, 12 red; 7 green, 8 red, 4 blue; 3 blue, 8 red; 6 green, 9 red, 3 blue; 11 green, 12 red";

    #[googletest::test]
    #[rstest]
    #[case(DRAWS_STR, expected_draws())]
    fn test_parser(#[case] draws_str: &str, #[case] expected: Draws) {
        let parsed_draws = super::parse(draws_str);
        expect_that!(parsed_draws, eq(&expected))
    }

    #[fixture]
    fn expected_draws() -> Draws {
        vec![
            Draw {
                blue: 5,
                red: 7,
                green: 0,
            },
            Draw {
                green: 2,
                blue: 1,
                red: 12,
            },
            Draw {
                green: 7,
                red: 8,
                blue: 4,
            },
            Draw {
                blue: 3,
                red: 8,
                green: 0,
            },
            Draw {
                green: 6,
                red: 9,
                blue: 3,
            },
            Draw {
                green: 11,
                red: 12,
                blue: 0,
            },
        ]
    }
}
