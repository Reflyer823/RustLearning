use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

// 使用match来传播错误
fn _read_username_from_file_1() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 使用 ? 运算符传播错误
fn _read_username_from_file_2() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// 链式调用
fn _read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// 在Option类型上使用 ? 运算符
fn _last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// 返回值不兼容的函数无法使用 ? 运算符
// fn main() {
//     let file = File::open("hello.txt")?;
// }

fn main() -> Result<(), Box<dyn Error>> {
    let _file = File::open("hello.txt")?;
    Ok(())
}
