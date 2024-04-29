#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("jack"),
        email: String::from("jackk@hotmail.com"),
        sign_in_count: 88,
    };
    println!("use is {:?}", user1);
    
    user1.email = String::from("jack@hotmail.com");
    println!("use is {:?}", user1);

    let user2: User = build_user(String::from("tom"), String::from("tom@hotmail.com"));
    println!("use is {:?}", user2);

    let user3 = User {
        email: String::from("jeery@example.com"),
        ..user2
    };
    println!("use is {:?}", user3);

    println!("usre3 active is {}, username is {}, sign in count is {}", user3.active, user3.username, user3.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black is {:?}", black);
    println!("origin is {:?}", origin);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);
    dbg!(rect1);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
