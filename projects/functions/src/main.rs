fn main() {
    // println!("Hello, world!"); // v1

    another_function(5); // v1 to v2
}

fn another_function(x: i32) { // rust does not care the location of functions. // .v2
    println!("The value of x is : {x}"); // .v1 to .v2
}
