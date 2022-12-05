fn main() {
    // 创建空vector
    let mut v1: Vec<i32> = Vec::new();
    // 使用vec!宏从初始值创建vector
    let v2 = vec![1, 2, 3];
    let v3 = vec![0; 6];
    dbg!(&v1, &v2, &v3);

    // 更新vector
    v1.push(5);
    v1.push(6);
    v1.push(7);

    // 访问vector元素：索引与get
    let third = &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 元素不存在：索引方法panic，get返回None
    // let does_not_exist = &v2[100];
    let does_not_exist = v2.get(100);
    dbg!(&does_not_exist);

    // 报错：不能同时存在可变和不可变的引用
    // let first = &v1[0];
    // v1.push(0);
    // println!("The first element is: {}", first);

    // 遍历vector中的元素
    let mut v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{}", i);
    }

    // 使用可变引用修改值
    for i in &mut v4 {
        *i += 50;
    }
    println!("{:?}", v4);

    // 从变量创建vector
    let n = 10;
    let mut v5 = vec![7; n];
    println!("{:?}", v5);

    // pop方法
    println!("{:?}", v5.pop());
    println!("{:?}", v5);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
