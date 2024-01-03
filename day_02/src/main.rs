use solver::solve;

mod draw;
mod game;
mod input;
mod solver;

fn main() {
    let result = solve(input::DATA);
    print!("The result of this riddle is {}!", result)
}
