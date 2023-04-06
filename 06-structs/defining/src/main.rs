struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: false,
        username: String::from("wjun"),
        email: String::from("test@hotmail.com"),
        sign_in_count: 1,
    };

    println!("user1 username is {}", user1.username);

    user1.email = String::from("demo@hotmail.com");

    println!(
        "user1 info: username -> {}, email -> {}, active -> {}, sign_in_count -> {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    let user2 = User {
        email: String::from("dodo@hotmail.com"),
        ..user1
    };

    println!("user2 email is {}", user2.email);

    let user3 = build_user(String::from("dodo"), String::from("koukou@hotmail.com"));

    println!("user3 email is {}", user3.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black value is {}", black.0);
    println!("origin value is {}", origin.0);

    let alway = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
