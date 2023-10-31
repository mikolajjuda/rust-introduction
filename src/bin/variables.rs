#![allow(uncommon_codepoints)]
#![allow(non_snake_case)]

fn takes_i64(_x: i64) {}

fn main() {
    let 𐐘: i32; // explicit type i32 (i32 is also the default for integers)
    let y = 10u8; // type inferred from literal u8
    let mut a = 1; // variables are immutable by default
    let b = 2;
    let _草泥马: (); // underscore prefix suppresses unused variable warning
    let _żółć: ();

    a += y + 1; // type of a inferred here u8
    takes_i64(b); // type of b inferred here i64
    𐐘 = 5; // variables can be initialised after declaration

    println!("𐐘 = {}", 𐐘);
    println!("y = {}", y);
    println!("a = {}", a);
    println!("b = {}", b);
}
