fn main() {
    let number = 5;

    // if后必须是bool类型，不匹配报错
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 多个else if可以用match替换
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    // if作为表达式
    let condition = true;
    let number = if condition { 4 } else { 5 };
    // 报错：if分支后的表达式必须接{}块
    // let number = if condition { 4 } else 5;
    println!("The value of number is {}", number);

    // loop循环表达式
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // while循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    // while循环遍历数组
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", arr[index]);
        index += 1;
    }

    // for循环遍历数组
    for element in arr.iter() {
        println!("the value is: {}",  element);
    }

    // for循环搭配Range使用
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
