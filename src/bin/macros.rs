macro_rules! print_items {
    ($($x:expr),*) => {
        $(
            println!("{}", $x);
        )*
    };
}

fn main() {
    let x = 5;
    print_items!(-2010, 12u8, 82, "a", x);
    print_items!("Hello", "World", "!");
}
