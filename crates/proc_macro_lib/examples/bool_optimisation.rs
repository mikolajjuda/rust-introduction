#![recursion_limit = "256"]

use proc_macro_lib::surround;

fn main() {
    println!("{}", std::mem::size_of::<bool>());
    println!("{}", std::mem::size_of::<Option<bool>>());
    println!("{}", std::mem::size_of::<surround!(254 "Option<" ">" bool)>());
    println!("{}", std::mem::size_of::<surround!(255 "Option<" ">" bool)>());
}
