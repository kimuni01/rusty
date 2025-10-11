struct Rectangle { // introducing a struct here v3
    width: u32,
    height: u32,
} // .v3

fn main() {
    // let width1 = 30; // v1
    // let height1 = 50;

    let rect1 = Rectangle { // v3
        width: 30,
        height: 50,
    }; // .v3

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        // area(rect1) // introducing a tuple here // v2 .v2
        area(&rect1)
    ); // .v1
}

// fn area(width: u32, height: u32) -> u32 { // v1
//     width * height
// } // .v1

// fn area(dimensions: (u32, u32)) -> u32 { // v2
//     dimensions.0 * dimensions.1
// } // .v2

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
