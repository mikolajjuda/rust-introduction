#![allow(uncommon_codepoints)]
#![allow(non_snake_case)]

// static items have a precise memory location
static NUM: i64 = 123;

// constants don't have a memory location and are inlined
const PI: f64 = 3.141592;

fn takes_i64(_x: i64) {}

fn main() {
    let 𐐘: i32; // explicit type i32 (i32 is also the default for integers)
    let y = 10u8; // type inferred from literal u8
    let mut a = 1; // variables are immutable by default
    let b = 2;
    let _żółć: (); // underscore prefix suppresses unused variable warning
    let 𓂺 = 100;

    a += y + 1; // type of a inferred here u8
    takes_i64(b); // type of b inferred here i64
    𐐘 = 5; // variables can be initialised after declaration

    println!("a = {}\n", a);
    let a = 0.5f64; // shadowing

    println!("𐐘 = {}", 𐐘);
    println!("y = {}", y);
    println!("a = {}", a);
    println!("b = {}", b);
    println!("𓂺   = {}", 𓂺);

    println!("PI = {}", PI);
    println!("NUM = {}", NUM);
}
