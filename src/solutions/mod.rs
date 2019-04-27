use lazy_static::lazy_static;
use std::collections::HashMap;

mod sol_1;
mod sol_10;
mod sol_2;
mod sol_25;
mod sol_292;
mod sol_3;
mod sol_4;
mod sol_48;
mod sol_494;
mod sol_5;
mod sol_6;
mod sol_69;
mod sol_7;
mod sol_73;
mod sol_8;
mod sol_9;

lazy_static! {
    pub static ref SOLVERS: HashMap<&'static str, fn() -> u64> = {
        let mut m = HashMap::new();

        m.insert("1", sol_1::solve as fn() -> u64);
        m.insert("2", sol_2::solve as fn() -> u64);
        m.insert("3", sol_3::solve as fn() -> u64);
        m.insert("4", sol_4::solve as fn() -> u64);
        m.insert("5", sol_5::solve as fn() -> u64);
        m.insert("6", sol_6::solve as fn() -> u64);
        m.insert("7", sol_7::solve as fn() -> u64);
        m.insert("8", sol_8::solve as fn() -> u64);
        m.insert("9", sol_9::solve as fn() -> u64);
        m.insert("10", sol_10::solve as fn() -> u64);
        m.insert("25", sol_25::solve as fn() -> u64);
        m.insert("48", sol_48::solve as fn() -> u64);
        m.insert("69", sol_69::solve as fn() -> u64);
        m.insert("73", sol_73::solve as fn() -> u64);
        m.insert("292", sol_292::solve as fn() -> u64);
        m.insert("494", sol_494::solve as fn() -> u64);

        m
    };
}
