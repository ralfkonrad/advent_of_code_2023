use crate::game::draw::Draw;
use crate::game::Game;

pub fn solve(max_draw: &Draw, input: &str) -> (u32, u32) {
    let result1 = input
        .lines()
        .map(Game::parse)
        .filter(|game| game.fits_into(max_draw))
        .map(|game| game.id)
        .sum();

    let result2 = input
        .lines()
        .map(Game::parse)
        .map(|game| game.get_minimal_possible_draw())
        .map(|draw| draw.power())
        .sum();

    (result1, result2)
}

#[cfg(test)]
mod tests {
    use crate::game::draw::MAX_DRAW;
    use crate::input::DATA;
    use crate::solver::solve;
    use googletest::prelude::*;

    #[googletest::test]
    fn test_input() {
        expect_that!(solve(&MAX_DRAW, DATA), eq((2061, 72596)))
    }
}
