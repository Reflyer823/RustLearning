fn main() {
    println!("Hello, world!");

    // 不写类型报错,不初始化使用报错
    let x: i32;
    x = 2;
    // x = 3;
    println!("The value of x is {}", x);

    // mut可变
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // const常量，类似constexpr，必须标注类型
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    // 可以改变类型
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces: {}", spaces);

    // parse方法返回值必须指定类型
    let guess: u32 = "42".parse().expect("parse error!");
    println!("Guess: {}", guess);

    // 整数字面值，进制及类型后缀
    let x1 = 98_222;
    let x2 = 0xffu8;
    let x3 = 0o77u32;
    let x4: i16 = 0b1110;
    let x5 = b'a';
    println!("x1: {}, x2: {}, x3: {}, x4: {}, x5: {}", x1, x2, x3, x4, x5);

    // 超出范围的字面值
    // 不加i64后缀默认i32，超出表示范围则报错
    let x6 = 1000000000000i64;
    println!("x6: {}", x6);
    // 下面均会报错
    // let x7 = 897i8;
    // let x8: i8 = 897;
    // 报错，不同类型不能赋值
    // let x9: i32 = 1000000000000i64;

    // 整数的溢出，debug下panic，release下循环
    // let x2 = x2 + 2;
    // println!("x2 = {}", x2);

    // 浮点类型
    let f1 = 2.0;
    let f2: f32 = 3.;
    let f3 = 5.6f32;
    println!("f1: {}, f2: {}, f3: {}", f1, f2, f3);

    // 负数取余测试
    println!("7 % 3 = {}", 7 % 3);
    println!("-7 % 3 = {}", -7 % 3);
    println!("7 % -3 = {}", 7 % -3);
    println!("-7 % -3 = {}", -7 % -3);

    // 布尔类型
    let b1 = true;
    println!("b1: {}", b1);

    // 字符类型
    let c1 = 'a';
    let c2 = '好';
    let c3 = '😊';
    println!("c1: {}, c2: {}, c3: {}", c1, c2, c3);
}
