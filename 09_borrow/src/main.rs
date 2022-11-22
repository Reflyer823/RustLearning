fn main() {
    let mut s1 = String::from("hello");

    // 传递不可变引用
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 传递可变引用
    modify_string(&mut s1);
    println!("s1: {}", s1);

    // 报错：不能同时借用两个可变引用
    // let r1 = &mut s1;
    // let r2 = &mut s1;
    // println!("r1: {}, r2: {}", r1, r2);

    // 允许非同时的创建多个可变引用
    {
        let r1 = &mut s1;
        println!("r1: {}", r1);
    }
    let r2 = &mut s1;
    println!("r2: {}", r2);

    // 报错：不能同时借用为可变和不可变
    // let r1 = &s1;
    // println!("r1: {}, r2: {}", r1, r2);

}

// 函数接收引用类型，不获得所有权
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 接受可变引用
fn modify_string(s: &mut String) {
    s.push_str(", world");
}

// 报错：悬垂引用，需要指定生命周期
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
