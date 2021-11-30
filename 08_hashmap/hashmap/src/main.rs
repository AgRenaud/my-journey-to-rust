use std::collections::HashMap;

fn main() {
    // Constructing with insert
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    // Constructing with iterator
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let field_name = String::from("Favorite color");
    let color = map.get(&field_name);

    if let Some(color) = color {
        println!("The favorite color is : {}", color);
    } else {
        println!("The favorite color doesn't exist in map");
    }

    println!("Iterate through map");
    for (key, value) in &map {
        println!("\t{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
