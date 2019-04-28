use env_logger::{Builder, Env};
use solutions::SOLUTIONS;
use std::env;

mod common;
mod solutions;

fn main() {
    Builder::from_env(Env::default().default_filter_or("info"))
        .default_format_timestamp(false)
        .init();

    let arg = env::args()
        .nth(1)
        .expect("Usage: `cargo run -- x`, where x is the problem id.");

    if let Some(solution) = SOLUTIONS.get(arg.as_str()) {
        solution();
    } else {
        println!("Problem {} isn't solved yet.", arg);
    }
}
