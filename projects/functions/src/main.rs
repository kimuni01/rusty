fn main() {
    // println!("Hello, world!"); // v1

    // another_function(5); // v1 to v2
    print_labeled_measurement(5, 'h'); // v3
}

// fn another_function(x: i32) { // rust does not care the location of functions. // .v2
//     println!("The value of x is : {x}"); // .v1 to .v2
// }

fn print_labeled_measurement(value: i32, unit_label: char) { // v3
    println!("The measurement is : {value}{unit_label}"); // .v3
}
