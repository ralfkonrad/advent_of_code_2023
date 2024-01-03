use crate::draw::Draw;
use crate::draw::Draws;
use regex::Regex;

#[derive(Debug)]
pub struct Game {
    pub number: u32,
    pub draws: Draws,
}

impl Game {
    pub fn parse(line: &str) -> Self {
        let regex = Regex::new(r"Game (?<number>\d+): (?<cubes>[\w\s,;]+)").unwrap();
        let captures = regex.captures(line).unwrap();
        let number = captures["number"].parse::<u32>().unwrap();
        let draws = Draw::parse(&captures["cubes"]);

        Self { number, draws }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Game;
    use googletest::prelude::*;
    use rstest::rstest;

    const GAME_11_STR: &str = "Game 11: 5 blue, 7 red; 2 green, 1 blue, 12 red; 7 green, 8 red, 4 blue; 3 blue, 8 red; 6 green, 9 red, 3 blue; 11 green, 12 red";
    const GAME_11: Game = Game {
        number: 11,
        draws: Vec::new(),
    };

    #[googletest::test]
    #[rstest]
    #[case(GAME_11_STR, GAME_11)]
    fn test_game_parser(#[case] line: &str, #[case] expected: Game) {
        let parsed_game = Game::parse(line);
        expect_that!(parsed_game.number, eq(expected.number));
        expect_that!(parsed_game.draws.len(), eq(expected.draws.len()));
    }
}
