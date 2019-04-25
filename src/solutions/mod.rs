use lazy_static::lazy_static;
use std::collections::HashMap;

mod sol_1;
mod sol_2;
mod sol_25;
mod sol_3;
mod sol_4;
mod sol_5;
mod sol_6;
lazy_static! {
    pub static ref SOLVERS: HashMap<&'static str, fn() -> u64> = {
        let mut m = HashMap::new();

        m.insert("1", sol_1::solve as fn() -> u64);
        m.insert("2", sol_2::solve as fn() -> u64);
        m.insert("3", sol_3::solve as fn() -> u64);
        m.insert("4", sol_4::solve as fn() -> u64);
        m.insert("5", sol_5::solve as fn() -> u64);
        m.insert("6", sol_6::solve as fn() -> u64);
        m.insert("25", sol_25::solve as fn() -> u64);

        m
    };
}
