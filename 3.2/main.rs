use std::io;

fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // 结果为 0

    // 取余
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // 显式指定类型注解

    // char 注意是:单引号
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (50, 6.24, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 数组
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // 变量名为 a 的数组将包含 5 个元素，这些元素的值最初都将被设置为 3。
    // 与let a = [3, 3, 3, 3, 3];相同
    let a = [3; 5];

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
