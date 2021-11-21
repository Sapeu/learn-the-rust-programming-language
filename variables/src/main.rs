fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", x + 10);

    // x = 10;
    // println!("The value of x is: {}", x);

    let mut x = 8;
    println!("The value of x is: {}", x);

    x = 10;
    println!("The value of x is: {}", x);

    const ONE_DAY_SECONDS: u32 = 24 * 60 * 60;
    println!("The value of one day is: {} seconds", ONE_DAY_SECONDS);

    let x = 111;
    let x = x + 222;

    {
        println!("The value of x in the inner scope is: {}", x);
        let x = x * 333;
        println!("The value of x * 333 in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "                     ";
    println!("The spaces string is: {}.", spaces);
    let spaces = spaces.len();
    println!("The spaces string length is: {}.", spaces);

    // let spaces = "                                ";
    // println!("The spaces string is: {}.", spaces);
    // spaces = spaces.len();
    // println!("The spaces string length is: {}.", spaces);

}
