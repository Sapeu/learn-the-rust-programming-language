fn main() {
    if_branches(3);
    if_branches(7);

    multiple_if_branches(6);

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

fn if_branches(number: i32) {
    if number < 5 {
        println!("{} condition was true", number);
    } else {
        println!("{} condition was false", number);
    }
}

fn multiple_if_branches(number: i32) {
     if number % 4 == 0 {
        println!("number {} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("number {} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("number {} is divisible by 2", number);
    } else {
        println!("number {} is not divisible by 4, 3, or 2", number);
    }
}
