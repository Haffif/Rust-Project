struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("admin@gmail.com"),
        username: String::from("admin"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        email: user1.email,
        username: user1.username,
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    let user3 = User {
        email: String::from("user@gmail.com"),
        ..user2
    };

    //user2.email = String::from("administrator@gmail.com");

    println!("Hello, {}", user2.email);
    println!("Hello, {}", user3.email);
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }