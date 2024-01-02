use crate::game::Game;

pub fn solve(input: &str) -> usize {
    input.lines().map(solve_line).filter(|b| *b).count()
}

fn solve_line(line: &str) -> bool {
    let _game = Game::parse(line);
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::input::DATA;
    use crate::solver::*;

    #[test]
    #[should_panic]
    fn test_input() {
        _ = solve(DATA)
    }
}
