use game::draw::MAX_DRAW;
use solver::solve;

mod game;
mod input;
mod solver;

fn main() {
    let (result1, result2) = solve(&MAX_DRAW, input::DATA);
    print!("The result of this riddle is {result1}, the extra result is {result2}!")
}
