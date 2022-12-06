fn main() {
    // 创建字符串
    let s = String::new();
    println!("{:?}", s);

    let s = "initial contents".to_string();
    println!("{:?}", s);

    let s = String::from("initial contents");
    println!("{:?}", s);

    // UTF8字符
    let hello = String::from("你好");
    println!("{:?}, {:?}", hello, hello.len());

    // 更新字符串
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("{:?}, {:?}", s, s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("{:?}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{:?}", s3);
    // 报错：s1不再有效
    // println!("{:?}", s1);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    // format!宏不获取任何所有权
    dbg!(&s, &s1, &s2, &s3);

    // 错误：字符串不能索引
    // let c = s[0];

    let hello = "Здравствуйте";
    dbg!(hello.len());

    let s = &hello[0..4];
    println!("{:?}", s);
    
    // panic：非字符边界
    // let s = &hello[0..1];

    // 字符形式遍历
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // 字节形式遍历
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
