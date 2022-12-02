use crate::garden::vegetables::Asparagus;
use std::collections::HashMap;

// 使用as别名避免冲突
// use std::fmt::Result;
// use std::io::Result as IoResult;

use rand::Rng;

// 嵌套路径
// use std::{cmp::Ordering, io};
// use std::io::{self, Write};

// 使用*引入所有公有项
// use std::collections::*;

pub mod garden;

fn main() {
    let plant = Asparagus{};
    println!("I'm growing {:?}!", plant);

    let mut map = HashMap::new();
    map.insert(1, 2);

    crates_and_modules::hosting::add_to_waitlist();

    let secret_number = rand::thread_rng().gen_range(1..=100);
    dbg!(secret_number);
}
