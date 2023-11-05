/**
 * This function adds two numbers.
 * # Examples
 * 
 * ```
 * let x = 5;
 * let y = 3;
 * assert_eq!(8, add_numbers(x, y));
 * ```
 * ## this is markdown text
 * **bold** *italic*
 * - list1
 * - list2
 */
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add_numbers() {
    assert_eq!(8, add_numbers(5, 3));
}

fn main() {
    let x = 5;
    let y = 3;
    println!("{} + {} = {}", x, y, add_numbers(x, y));
}
