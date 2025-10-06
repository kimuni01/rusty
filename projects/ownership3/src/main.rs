fn main() { // v1
    // let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("The length of '{}' is {}.", s2, len);

    // let len = calculate_length(&s1); // v2
    // println!("The length of '{}' is {}.", s1, len); // .v2

    // let s = String::from("hello"); // v3
    // change(&s); // .v3

    let mut s = String::from("hello"); // v4
    change(&mut s); // .v4 // see how this does not error even in v7

    // let r1 = &mut s; // v5
    // let r2 = &mut s; // mutable reference more than one cause compiler error

    // println!("{}, {}", r1, r2); // .v5

    // { // v6
    //     let r1 = &mut s;
    // } // from here r1 is out of the scope, therefore making a new reference is possible
    // let r2 = &mut s; // .v6

    // let r1 = &s; // no problem // v7
    // let r2 = &s; // no problem
    // let r3 = &mut s; // significant problem
    // println!("{}, {} and {}", r1, r2, r3); // .v7

    let r1 = &s; // no problem // v8
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // since this point, variable r1 and r2 are not used.

    let r3 = &mut s; // no problem
    println!("{}", r3); // .v8
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of String.

//     (s, length)
// } // .v1

// fn calculate_length(s: &String) -> usize { // s is the reference of String. // v2
    // s.len();
// } // from here s is out of the scope. however it does not own what it refers, // .v2
// hence original String is not dropped.

// fn change(some_string: &String) { // v3
//     some_string.push_str(", world"); // this cannot work, reference cannot change the value
// } // .v3

fn change(some_string: &mut String) { // v4
    some_string.push_str(", world");
} // .v4
