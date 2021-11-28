#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    println!("Hello, world!\n{:?}", v);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("Hello, world!\n{:?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(5) {
        Some(five) => println!("The five element is {}", five),
        None => println!("There is no third element."),
    }

    // let first = &v[0];
    // v.push(9);
    // println!("The third element is {}", first);

    // dbg!(&v[100]);
    // dbg!(v.get(100));

    for i in &mut v {
        dbg!(*i *= 33);
    }
    dbg!(v);

    let mut row = vec![
        SpreadsheetCell::Int(333),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(12.34),
    ];

    dbg!(&row);
    let row_last_value = dbg!(row.pop());
    if let Some(s) = row_last_value {
        if let SpreadsheetCell::Float(f) = s {
            dbg!(f);
        };
    };
    dbg!(&row);
}
