fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{:?}", vec);

    let mut string: String = "Hello".to_string();
    string.push_str(" world");
    string.push('!');
    println!("{}", string);
}
