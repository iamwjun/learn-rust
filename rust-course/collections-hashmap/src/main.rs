use std::collections::HashMap;

fn main() {
    let mut my_gems = HashMap::new();

    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);

    println!("my_gems is {:#?}", my_gems);
}
