#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let width1 = 30;
    let heigth1 = 20;
    println!("this area of rectangle is {} square pixels", area(width1, heigth1));

    let rect1 = (32, 20);

    println!("this area of rectangle is {} square pixels", re_area(rect1));

    let rect2 = Rectangle {
        width: 32,
        height: 20
    };

    println!("this area of rectangle is {} square pixels", range_area(&rect2));

    println!("rect2 is {:#?}", rect2);

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 30
    };
    dbg!(&rect3);
}

fn area(width: u32, heigth: u32) -> u32 {
    width * heigth
}

fn re_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn range_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
