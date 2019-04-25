use solutions::*;
use std::env;

mod common;
mod solutions;

fn main() {
    const USAGE: &str = "To run a solution, execute `cargo run -- x`, where x is the problem id.";
    let arg = env::args().nth(1).expect(USAGE);

    let solution = match arg.as_str() {
        "1" => solve_1(),
        "2" => solve_2(),
        "3" => solve_3(),
        "4" => solve_4(),
        "5" => solve_5(),
        _ => panic!(USAGE),
    };

    println!("{}", solution);
}
