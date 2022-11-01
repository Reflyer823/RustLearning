fn main() {
    println!("Hello, world!");
    another_function(5);

    let y = {
        let x = 1;
        x + 3
    };
    println!("The value of y is {}", y);

    let x = five();
    println!("The value of x is {}", x);

    let x = plus_five(6);
    println!("The value of x is {}", x);

    // 无返回值的函数返回()，打印会报错
    // let res = another_function(5);
    // println!("res: {}", res);
}

fn another_function(x: i32) {
    println!("Another function!");
    println!("The value of x is {}", x);
}

fn five() -> i32 {
    5
}

fn plus_five(x: i32) -> i32 {
    x + 5
}