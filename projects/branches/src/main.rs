fn main() {
    // let number = 3; // v1

    // if number < 5 {
    //     println!("condition was true");
    // }
    // else {
    //     println!("condition was false");
    // } // .v1

    // if number != 0 { // v2
    //     println!("number was something other than zero");
    // } // .v2

    let number = 6; // v3

    if number % 4 == 0 {
        println!("number is divisible by 4");
    }
    else if number % 3 == 0 {
        println!("number is divisible by 3");
    }
    else if number % 2 == 0 {
        println!("number is divisible by 2");
    }
    else {
        println!("number is not divisible by 4, 3 or 2");
    } // .v3
}
