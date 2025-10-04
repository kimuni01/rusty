fn main() { // 's' is not yet defined, hence invalid at here. // v1
    // let s = "hello"; // since this point, 's' is valid. // .v1

    // do something with 's'.

    let mut s = String::from("hello"); // v2
    s.push_str(", world!"); // push_str() adds literal to the String.
    println!("{}", s); // this lline prints 'hello, world!' // .v2
} // this scope has ended, 's' is no longer valid.
