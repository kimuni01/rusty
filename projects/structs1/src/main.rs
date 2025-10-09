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

    // let user2 = User { // v5
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // }; // .v5

    let user2 = User { // v6
        email: String::from("another@example.com"),
        ..user1 // uses = operator, which moves reference type values
    }; // .v6

}

fn build_user(email: String, username: String) -> User { // v3
    User {
        active: true,
        // username: username,
        // email: email,
        sign_in_count: 1,
        username, // v4
        email, // .v4
    } // no semicolon for an expression
} // .v3
