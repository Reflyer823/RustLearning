fn main() {
    // String切片
    let s = String::from("hello world");
    let s1 = &s[0..5];
    let s2 = &s[6..11];
    println!("{} {}", s1, s2);

    let mut s = String::from("hello world");
    let word = first_word(&s);
    // 报错，不能同时借用为可变和不可变
    // s.clear();
    println!("the first word is: {}", word);

    // 字符串字面值就是切片
    let my_string_literal = "Hello, World!";
    let word = first_word_slice(my_string_literal);
    println!("{}", word);
    let word = first_word_slice(&s[..]);
    println!("{}", word);

    // 其他类型的slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// 首个单词的切片
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// 更改参数为字符串切片类型&str
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

