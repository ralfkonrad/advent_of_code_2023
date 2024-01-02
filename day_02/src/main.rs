use solver::solve;

pub mod game;
pub mod input;
pub mod solver;

fn main() {
    let result = solve(input::DATA);
    print!("The result of this riddle is {}!", result)
}
