mod engine_schematic;
mod input;
mod input_to_2darray;
mod solver;

use crate::input::DATA;
use crate::solver::solve;

fn main() {
    let (result1, result2) = solve(DATA);
    print!("The result of this riddle is {result1}, the extra result is {result2}!")
}
