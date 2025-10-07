fn main() {
    // let reference_to_nothing = dangle(); // v1 .v1
    let real_string = no_dangle(); // v2
}

// fn dangle() -> &String { // dangle returns reference of String // v1
    // let s = String::from("hello"); // s is a new String

    // &s //returns a reference of String
// } // on here s is out of scope and dropped. its memory is released // .v1
// this is dangerous

fn no_dangle() -> String { // v2
    let s = String::from("hello");

    s
} // .v2
