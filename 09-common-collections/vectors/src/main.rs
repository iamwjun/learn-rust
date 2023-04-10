fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);

    println!("v is {:#?}", v);

    let v2 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v2[2];

    println!("third is {third}");

    let fifth: Option<&i32> = v2.get(4);

    match fifth {
        Some(fifth) => println!("fifth element is {fifth}"),
        None => println!("no fifth element!")
    }

    // let does_not_exist = &v2[100];
    let does_not_exist = v2.get(100);

    println!("no fifth element {:#?}!", does_not_exist);

    let mut v3 = vec![1, 2, 3, 4, 5];

    // Changes reallocate memory for the vector
    v3.push(6);

    let first = &v3.get(0);
    
    println!("the first element is {:#?}!", first);

    let v4 = vec![100, 99, 98];
    
    for i in v4 {
        println!("this element is {}!", i);
    }

    let mut v5 = vec![100, 200, 300];

    for i in &mut v5 {
        *i += 1;
    }

    println!("v5 is {:?}!", v5);

    #[derive(Debug)]
    enum SpreedsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let v6 = vec![
        SpreedsheetCell::Int(32),
        SpreedsheetCell::Float(3.6),
        SpreedsheetCell::Text(String::from("title"))
    ];

    println!("v6 is {:#?}!", v6);
}
