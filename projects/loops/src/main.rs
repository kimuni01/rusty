fn main() {
    // loop { // has to be quit with Ctrl + C in terminal // v1
        // println!("again!");
    // } // .v1

    let mut counter = 0; // v2

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // this is how to return a value of a loop
        }
    };

    println!("The result is {result}"); // .v2
}
