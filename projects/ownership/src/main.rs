fn main() { // 's' is not yet defined, hence invalid at here. // v1
    // let s = "hello"; // since this point, 's' is valid. // .v1

    let s = String::from("hello"); // since this point, 's' is valid. // v3

    let x = 5;
    let y = x;
    let s1 = String::from("hello");
    let s2 = s1; // .v3

    println!("{}, world!", s1); // useful compiler error description // v4 .v4

    // do something with 's'.

    // let mut s = String::from("hello"); // v2
    // s.push_str(", world!"); // push_str() adds literal to the String.
    // println!("{}", s); // this lline prints 'hello, world!' // .v2


} // this scope has ended, 's' is no longer valid.
