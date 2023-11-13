use std::collections::HashMap;

fn main() {
    let mut map = HashMap::<i32, String>::new();
    map.insert(123, String::from("a"));
    map.insert(321, String::from("b"));
    
    match map.get(&666) {
        Some(n) => println!("{}", n),
        None => println!("No such key")
    }

    let x: &String = map.get(&123).unwrap(); // panics if None
    println!("{}", x);
    println!("{}", map[&321]); // panics if no such key
    println!("{}", map.get(&1000).expect("oops")); // panics on None with a message
}
