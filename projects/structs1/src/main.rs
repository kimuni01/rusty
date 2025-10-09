struct User { // v1
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} // .v1

fn main() {
    let user1 = User {  // v2
        sign_in_count: 1,
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
    }; // .v2

    let mut user1 = User { // v3
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com"); // .v3
}

fn build_user(email: String, username: String) -> User { // v3
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    } // no semicolon for an expression
} // .v3
