use solver::solve;

mod input;
mod solver;

fn main() {
    let (result1, result2) = solve(input::DATA);
    print!("The result of this riddle is {result1}, the extra result is {result2}!")
}
