use solutions::SOLUTIONS;
use std::env;

mod common;
mod solutions;

fn main() {
    let arg = env::args()
        .nth(1)
        .expect("Usage: `cargo run -- x`, where x is the problem id.");

    if let Some(solution) = SOLUTIONS.get(arg.as_str()) {
        solution();
    } else {
        println!("Problem {} isn't solved yet.", arg);
    }
}
