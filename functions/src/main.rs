fn main() {
    println!("Hello, world!");

    another_function();
    f_value(222);
    print_labeled_measurement(1, 'o');

    let x = 5;

    let y = {
        println!("The value of x is: {}", x);
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let five = five();
    println!("The value of five is: {}", five);
    let plus_one = plus_one(five);
    println!("The value of plus_one is: {}", plus_one);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn f_value(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}
