use std::collections::HashMap;

fn main() {
    let mut my_gems = HashMap::new();

    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);

    println!("my_gems is {:#?}", my_gems);

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let mut teams_map = HashMap::new();
    for team in &teams_list {
        teams_map.insert(&team.0, team.1);
    }

    println!("{:?}",teams_map);

    let teams_map: HashMap<String, i32> = teams_list.into_iter().collect();
    
    println!("{:?}",teams_map);

    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(name, age);

    println!("insert name {:?}", handsome_boys);
    println!("insert age is {}", age);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let v = scores.entry("Yellow".to_owned()).or_insert(5);

    println!("v is {}", v);

    let text = "hello world wonderful world";
    let mut map =  HashMap::new();

    for work in text.split_whitespace() {
        let count  = map.entry(work).or_insert(0);
        *count += 1;
    }

    println!("map is {:?}", map);
}
