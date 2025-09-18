fn main() {
    // println!("Hello, world!"); // v1

    // another_function(5); // v1 to v2
    // print_labeled_measurement(5, 'h'); // v3

    // let x = (let y = 6); // this is making an error and a warning // v4

    let y = { // in Rust, expressions and statements are differentiated // v5
        let x = 3; // this line is a statement, not returning any value
        x + 1 // without this line, y is apparently empty, causing error for println!
    }; // expressions must not end with a semicolon(;), otherwise become statements

    println!("The value of y is : {y}"); // .v5
}

// fn another_function(x: i32) { // rust does not care the location of functions. // .v2
//     println!("The value of x is : {x}"); // .v1 to .v2
// }

// fn print_labeled_measurement(value: i32, unit_label: char) { // v3
//     println!("The measurement is : {value}{unit_label}"); // .v3
// }
