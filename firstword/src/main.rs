fn main() {
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    println!("{:?}", hello);
    println!("{:?}", &s[..5]);
    let world = &s[6..11];
    println!("{:?}", world);
    println!("{:?}", &s[6..]);
    println!("{:}", &s[..]);
    let word = first_word(&s);
    println!("{}", word);
    s.clear();

    let s1 = "Welcome to Rust!";
    println!("{}", first_word(s1));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    println!("{:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
