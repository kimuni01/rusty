fn main() {
    // let reference_to_nothing = dangle(); // v1 .v1
    // let real_string = no_dangle(); // v2

    // let mut s = String::from("hello world"); // v4
    // let word = first_word(&s); // word receives value 5
    // s.clear(); // this code empties String and make ""
    // from here, word still has 5 but the String that lets 5 usable
    // does not exist. word is not valid at all now // .v4


    // let s = String::from("hello world"); // v5
    // let hello = &s[0..5];
    // let world = &s[6..11];

    // let len = s.len();
    // let slice = &s[0..len]; // meaning 'to start from first to the last'
    // let slice = &s[..]; // meaning the same as above // .v5


    let mut s = String::from("hello world"); // v6
    let word = first_word(&s); // immutable borrow occurs
    s.clear(); // error, creates a mutable borrow
    println!("the first word is : {}", word); // .v6

}

// fn dangle() -> &String { // dangle returns reference of String // v1
    // let s = String::from("hello"); // s is a new String

    // &s //returns a reference of String
// } // on here s is out of scope and dropped. its memory is released // .v1
// this is dangerous

// fn no_dangle() -> String { // v2
    // let s = String::from("hello");

    // s
// } // .v2

// fn first_word(s: &String) -> usize { // v3
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// } // .v3

fn first_word(s: &String) -> &str { // v6
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
} // .v6
