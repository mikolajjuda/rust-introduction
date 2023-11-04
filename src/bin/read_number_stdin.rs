fn main() {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let trimmed: &str = input.trim();
            match trimmed.parse::<i8>() {
                Ok(i) => println!("your integer input: {}", i),
                Err(e) => println!("parsing error: {}", e),
            }
        }
        Err(_) => panic!("error: unable to read line from stdin"),
    }
}
