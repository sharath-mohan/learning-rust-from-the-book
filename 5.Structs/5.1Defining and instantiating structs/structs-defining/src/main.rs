struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    // this struct is immutable
    let user1 = User {
        active: true,
        username: String::from("sharath mohan"),
        email: String::from("sharath@email.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.username);
    // this struct is mutatble
    let mut user2 = User {
        active: true,
        username: String::from("Mohan"),
        email: String::from("mohan@email.com"),
        sign_in_count: 1,
    };
    user2.sign_in_count = 2;
    println!("{}", user2.sign_in_count);
    user2.email = String::from("sharathmohan@email.com");
    println!("{}", user2.email);
    let user3 = build_user(String::from("david@email.com"), String::from("david"));
    println!("{}", user3.active);
    let user4 = build_user_short_hand(String::from("Peter"), String::from("peter@email.com"));
    println!("{}", user4.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
fn build_user_short_hand(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
