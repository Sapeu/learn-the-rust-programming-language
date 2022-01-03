fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("map:{:?}", scores);
    scores.insert(String::from("Blue"), 20);
    println!("map:{:?}", scores);

    let teams = vec![String::from("Pink"), String::from("Green")];
    let initial_scores = vec![10, 50];
    let scs: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("map:{:?}", scs);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map:{:?}", map);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("map:{:?}", score);

    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    scores.entry(String::from("Gray")).or_insert(2);
    scores.entry(String::from("Black")).or_insert(1);
    println!("map:{:?}", scores);

    let text = "hello world wonderful world";
    let mut map  = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map:{:?}", map);
}
