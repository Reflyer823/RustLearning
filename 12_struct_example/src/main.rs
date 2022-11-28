// 派生Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rect1 is: {}", area(&rect1));

    // 报错，未实现Display
    // println!("rect1 is: {}", rect1);

    // 打印调试信息
    println!("rect1 is: {:?}", rect1);

    // 美观格式输出
    println!("rect1 is: {:#?}", rect1);

    // dbg!宏，会打印文件、行号和内容至stderr流中
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);

}

// 长方形面积函数
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}