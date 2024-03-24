#![allow(dead_code, unused)]

fn main() {
    // 从字面值创建String
    let mut s1 = String::from("Hello");
    s1.push_str(", World");
    println!("{}", s1);

    // s1移动给s2后，s1不能再被访问
    // let s2 = s1;
    // println!("{}", s1);

    // 使用clone创建拷贝，拷贝后两个都能使用
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);

    // 实现了Copy Trait，可以拷贝
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);

    // s被移动到函数内
    let s = String::from("Hello, World");
    take_ownership(s);

    // x的副本传入函数
    let x = 5;
    makes_copy(x);
    println!("x: {}", x);

    // 函数内的对象交回所有权
    let s3 = gives_ownership();

    // s4所有权交给函数，再交还给s5
    let s4 = String::from("hello");
    let s5 = takes_and_gives_back(s4);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string 
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
