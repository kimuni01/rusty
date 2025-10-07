fn main() {
    let reference_to_nothing = dangle(); // v1
}

fn dangle() -> &String { // dangle returns reference of String
    let s = String::from("hello"); // s is a new String

    &s //returns a reference of String
} // on here s is out of scope and dropped. its memory is released // .v1
// this is dangerous