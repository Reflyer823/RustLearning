fn main() {
    // 定义tuple
    // let tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 解构
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    // 访问tuple元素
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    // 定义数组
    let mut arr1:[u32; 5] = [1, 2, 3, 4, 5];
    let arr2 = ["Hello", "World"];
    let arr3 = [3; 5];

    // 数组索引
    arr1[0] = 9;
    println!("{}", arr1[0]);
    println!("{}", arr2[0]);
    println!("{}", arr3[4]);

    // 数组越界,简单越界编译报错
    // println!("{}", arr3[10]);
    // 运行时越界panic
    // let index = [9, 10, 11, 12];
    // println!("{}", arr3[index[0]]);

    // 长度只能使用const,用let报错
    const N: usize = 10;
    let val = 0;
    let arr4 = [val; N];
    println!("{}", arr4[0]);
}
