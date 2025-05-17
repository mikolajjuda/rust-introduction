fn print_str(s: &str) {
    println!("{}", s);
}
fn first_four_letters(s: &String) -> &str {
    &s[..4] /* caution:
            byte indexing, use with ascii only*/
}
fn square_slice(s: &mut [i32]) {
    for i in s {
        *i = *i * *i;
    }
}
fn main() {
    let s = "Hello, world!".to_owned();
    println!("{}", first_four_letters(&s));
    print_str(&s[7..=11]);

    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8];
    square_slice(&mut arr[3..]); /* caution:
                                 byte indexing, use with ascii only*/
    println!("{:?}", arr);
}
