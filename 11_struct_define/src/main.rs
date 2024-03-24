#![allow(dead_code, unused)]

// 结构体定义
struct User {
    active: bool,
    // 报错：需要生命周期
    // username: &str,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 元组结构体
// 这两个都是3个i32，但是不同类型
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
struct AlwaysEqual;

fn main() {
    // 创建实例
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 实例可变时，所有字段都可变
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    // 更新语法创建实例
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // 报错，此时user1的username已被move
    // println!("{}", user1.username);

    // 元组结构体的实例和访问
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}, {}, {}", black.0, black.1, black.2);

    // 类单元结构体的实例
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    // 字段名和变量名相同时可省略
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
