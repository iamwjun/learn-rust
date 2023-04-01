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
        sign_in_count: 1,
    };

    println!("user1 username is {}", user1.username);

    user1.email = String::from("demo@hotmail.com");

    println!("user1 email is {}", user1.email);

    let user2 = User {
        email: String::from("dodo@hotmail.com"),
        ..user1
    };

    let user3 = build_user(String::from("dodo"), String::from("koukou@hotmail.com"));
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
