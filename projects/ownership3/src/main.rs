fn main() { // v1
    let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("The length of '{}' is {}.", s2, len);

    let len = calculate_length(&s1); // v2
    println!("The length of '{}' is {}.", s1, len); // .v2
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of String.

//     (s, length)
// } // .v1

fn calculate_length(s: &String) -> usize { // s is the reference of String. // v2
    s.len();
} // from here s is out of the scope. however it does not own what it refers, // .v2
// hence original String is not dropped.
