#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("jack"),
        email: String::from("jackk@hotmail.com"),
        sign_in_count: 88,
    };
    println!("use is {:?}", user1);
    
    println!("use is {:?}", user1);

    let user2: User = build_user(String::from("tom"), String::from("tom@hotmail.com"));
    println!("use is {:?}", user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
