use crate::game::draw::Draw;
use crate::game::draw::Draws;
use regex::Regex;

pub mod draw;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Game {
    pub id: u32,
    pub draws: Draws,
}

impl Game {
    pub fn parse(line: &str) -> Self {
        let regex = Regex::new(r"Game (?<id>\d+): (?<cubes>[\w\s,;]+)")
            .expect("This should be a valid regular expression");
        let captures = regex.captures(line).expect("This should be not a game");
        let id = captures["id"]
            .parse::<u32>()
            .expect("A game should hve an ID");
        let draws = Draw::parse(&captures["cubes"]);

        Self { id, draws }
    }

    pub fn fits_into(&self, max_draw: &Draw) -> bool {
        self.draws.iter().all(|draw| draw.fits_into(max_draw))
    }

    pub fn get_minimal_possible_draw(&self) -> Draw {
        let max_blue = self
            .draws
            .iter()
            .map(|draw| draw.blue)
            .max()
            .expect("Blue should be defined");
        let max_red = self
            .draws
            .iter()
            .map(|draw| draw.red)
            .max()
            .expect("Red should be defined");
        let max_green = self
            .draws
            .iter()
            .map(|draw| draw.green)
            .max()
            .expect("Green should be defined");
        Draw {
            blue: max_blue,
            red: max_red,
            green: max_green,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::draw::Draw;
    use crate::game::Game;
    use googletest::prelude::*;
    use rstest::{fixture, rstest};

    const GAME_11_STR: &str = "Game 11: 5 blue, 7 red; 2 green, 1 blue, 12 red; 7 green, 8 red, 4 blue; 3 blue, 8 red; 6 green, 9 red, 3 blue; 11 green, 12 red";

    #[googletest::test]
    #[rstest]
    #[case(GAME_11_STR, expected_game_11())]
    fn test_game_parser(#[case] line: &str, #[case] expected: Game) {
        let parsed_game = Game::parse(line);
        expect_that!(parsed_game.id, eq(expected.id));
        expect_that!(parsed_game.draws.len(), eq(expected.draws.len()));
        expect_that!(parsed_game, eq(expected))
    }

    #[googletest::test]
    #[rstest]
    #[case(GAME_11_STR, Draw { blue: 5, red: 12, green: 11 })]
    fn test_minimal_possible_draw(#[case] line: &str, #[case] expected: Draw) {
        let parsed_game = Game::parse(line);
        let minimal_possible_draw = parsed_game.get_minimal_possible_draw();
        expect_that!(minimal_possible_draw, eq(expected))
    }

    #[fixture]
    fn expected_game_11() -> Game {
        let draws = vec![
            Draw {
                blue: 5,
                red: 7,
                green: 0,
            },
            Draw {
                blue: 1,
                red: 12,
                green: 2,
            },
            Draw {
                blue: 4,
                red: 8,
                green: 7,
            },
            Draw {
                blue: 3,
                red: 8,
                green: 0,
            },
            Draw {
                blue: 3,
                red: 9,
                green: 6,
            },
            Draw {
                blue: 0,
                red: 12,
                green: 11,
            },
        ];
        Game { id: 11, draws }
    }
}
