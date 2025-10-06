fn main() { // v1
    let s1 = gives_ownership(); // gives_ownership moves it's return value to s1.
    let s2 = String::from("hello"); // s2 comes into the scope.
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back,
    // this method also moves its return value to s3.
} // from here, s3 is out of scope and dropped. nothing happens to s2 as it is
// already moved. s1 is out of scope and dropped.

fn gives_ownership() -> String { // gives_ownership moves it's return value
    // to its caller method.

    let some_string = String::from("yours"); // some_string comes into the scope.

    some_string // some_string is returned then moves to caller method.
}

// this method takes String then returns the same.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into the
    // scope.

    a_string // a_string is returned then moves to caller method.
} // .v1
