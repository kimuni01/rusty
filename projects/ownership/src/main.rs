fn main() { // 's' is not yet defined, hence invalid at here. // v1
    // let s = "hello"; // since this point, 's' is valid. // .v1

    let s = String::from("hello"); // since this point, 's' is valid. // v3

    // let x = 5;
    // let y = x;
    // println!("x = {}, y = {}", x, y); // v5 .v5
    // let s1 = String::from("hello");
    // let s2 = s1; // .v3
    // let s2 = s1.clone(); // v5 .v5

    // println!("{}, world!", s1); // useful compiler error description // v4 .v4
    // println!("s1 = {}, s2 = {}", s1, s2); // v5 .v5

    // do something with 's'.

    // let mut s = String::from("hello"); // v2
    // s.push_str(", world!"); // push_str() adds literal to the String.
    // println!("{}", s); // this lline prints 'hello, world!' // .v2


    takes_ownership(s); // value of s is moved inside the method // v6
    // hence it is no longer valid on here.
    let x = 5; // x comes inside the scope.
    makes_copy(x); // x is moved inside the method but,
    // i32 is Copy so x shall be used afterwards.
} // this scope has ended, 's' is no longer valid.
// so is x. however value of s is moved, nothing special happens. // .v6

fn takes_ownership(some_string: String) { // some_string comes inside the scope. // v6
    println!("{}", some_string);
} // from here some_string is out of scope and 'drop' is called.
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes inside the scope.
    println!("{}", some_integer);
} // from here some_integer is out of scope. nothing special happens. // .v6
