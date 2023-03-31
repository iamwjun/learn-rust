struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: false,
        username: String::from("wjun"),
        email: String::from("test@hotmail.com"),
        sign_in_count: 1
    };

    println!("user1 username is {}", user1.username);

    user1.email = String::from("demo@hotmail.com");

    println!("user1 email is {}", user1.email);
}
