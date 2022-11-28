#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 可以定义和字段同名的方法
    // 通常作为Getter使用
    fn width(&self) -> bool {
        self.width > 0
    }
}

// 可以有多个impl块
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rect1 is: {}", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let sq = Rectangle::square(3);
    dbg!(&sq);
}
