fn main() {
    let mut s = String::new();
    dbg!(s);

    let data = dbg!("initial contents");

    let s = dbg!(data.to_string());

    // 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    let mut s = String::from("foo");
    dbg!(s.push_str("bar"));
    dbg!(s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1, s2 is {}, {}", s1, s2);

    let mut s = String::from("lo");
    s.push('l');
    dbg!(s);
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    dbg!(s3);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
    dbg!(s);

    let s1 = String::from("hello");
    // let h = s1[0];
    // dbg!(h);

    let len = String::from("Hola").len();
    dbg!(len);

    let len = String::from("Здравствуйте").len();
    dbg!(len);

    let hello = "Здравствуйте";
    // let answer = &hello[0];
    // dbg!(answer);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
