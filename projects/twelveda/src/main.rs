// a program for printing lyrics of the song
// "The Twelve Days of Christmas", utilizing it's repetitiveness
// 1st attempt, the only missed part is range(?..?), in fact there
// is no range() but only (?..?) exists.

fn main() {
    let lyr = ["And a partridge in a pear tree.",
    "Two turtle doves,", "Three French hens,", "Four calling birds,",
    "Five golden rings,", "Six geese a-laying,", "Seven swans a-swimming,",
    "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,",
    "Eleven pipers piping,", "Twelve drummers drumming,"];

    let lyr2 = ["first", "second", "third,", "fourth",
    "fifth", "sixth", "seventh", "eighth", "nineth", "tenth", "eleventh",
    "twelveth"];
    println!("The Twelve Days of Christmas\n");
    println!("On the first day of Christmas,\n
    My true love sent to me\n
    A partridge in a pear tree.");
    for i in (1..=11) {
        println!("On the {} day of Christmas,", lyr2[i]);
        println!("My true love sent to me");
        for j in (0..i).rev() {
            println!("{}", lyr[j]);
        }
    }
}
