fn main() {
    // loop {
    //     println!("again!");
    // }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 11 {
            break counter * 22;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the index {} value is: {}", index, a[index]);

        index = index + 1;
    }

    for element in a.iter() {
        println!("for the value is: {}", element);
    }

    for number in (1..10).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    println!("fib(20) is {}", fib(20));
    println!("fib(2) is {}", fib(2));
    println!("fib(1) is {}", fib(1));
    println!("fib(0) is {}", fib(0));
}

fn fib(n: i32) -> i32 {
    if n <= 0 {
        return 0
    }
    let mut num1 = 0;
    let mut num2 = 1;
    for _ in 1..n {
        let temp = num1 + num2;
        num1 = num2;
        num2 = temp;
    }
    num2
    // if (n <= 0){
    //    0 
    // } else if (n == 1 || n == 2) {
    //     1
    // } else {
    //     return fib(n - 1) + fib(n - 2)
    // }
}
