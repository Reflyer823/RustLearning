use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_result = File::open("hello.txt");
    let _file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // 使用闭包和unwrap_or_else实现相同功能
    let _file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let _file = File::open("hello.txt").unwrap();

    let _file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
