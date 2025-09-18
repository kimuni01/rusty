fn main() {
    // println!("Hello, world!"); // v1

    // another_function(5); // v1 to v2
    // print_labeled_measurement(5, 'h'); // v3

    // let x = (let y = 6); // this is making an error and a warning // v4

    // let y = { // in Rust, expressions and statements are differentiated // v5
    //    let x = 3; // this line is a statement, not returning any value
    //    x + 1 // without this line, y is apparently empty, causing error for println!
    // }; // expressions must not end with a semicolon(;), otherwise become statements

    // println!("The value of y is : {y}"); // .v5

    // let x = five(); // v6
    // println!("The value of x is : {x}"); // .v6

    let x = plus_one(5); // v7
    println!("The value of x is : {x}");
} // .7

// fn another_function(x: i32) { // rust does not care the location of functions. // .v2
//     println!("The value of x is : {x}"); // .v1 to .v2
// }

// fn print_labeled_measurement(value: i32, unit_label: char) { // v3
//     println!("The measurement is : {value}{unit_label}"); // .v3
// }

// fn five() -> i32 { // should a function return a value, it has to write "-> (type)" // v6
//     5 // no 'function calls', no 'macros', no 'let statement'. and this is perfectly fine.
// } // could have "return" before end of the function but most funcs end with an expression .v6

fn plus_one(x: i32) -> i32 { // v7
    x + 1
//    x + 1; // making an error, as this is a statement // this at the end makes func return ()
} // .v7
