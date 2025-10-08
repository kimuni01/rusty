struct User { // v1
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} // .v1

fn main() { // v2
    let user1 = User {
        sign_in_count: 1,
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
    };
} // .v2
