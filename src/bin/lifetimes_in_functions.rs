fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "xyz".to_string();
    let s2 = "abcd".to_string();
    let result = longest_str(s1.as_str(), s2.as_str());
    println!("The longest string is {}", result);
}
