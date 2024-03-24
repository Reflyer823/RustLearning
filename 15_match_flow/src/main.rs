#![allow(dead_code, unused)]

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("{}", value_in_cents(coin));

    value_in_cents(Coin::Quarter(UsState::Alabama));

    // Option枚举
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}, {:?}", five, six, none);

    let dice_roll = 9;
    match dice_roll {
        3 => println!("add_fancy_hat"),
        7 => println!("remove_fancy_hat"),
        // 通配分支
        // other => println!("move_player: {}", other),
        // 使用下划线表示不使用
        _ => (),
    }
}
