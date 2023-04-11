use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);

    println!("scores map is {:?}!", scores);

    let team_name = String::from("red");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{team_name} score is {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("map is {:?}!", map);

    map.insert(String::from("Favorite color"), String::from("white"));

    println!("map is {:?}!", map);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map: HashMap<String, i32> = HashMap::new();

    for world in text.split_whitespace() {
        println!("world is {world}");
        let count = map.entry(world.to_string()).or_insert(0);
        *count += 1;
        println!("{:?}", map);
    }

    map.entry(String::from("hello")).or_insert(0);

    println!("{:?}", map);
}
