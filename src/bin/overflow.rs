fn main() {
    println!("225u8 + 1 = {}", 255u8 + 1); // compile error in debug mode; 0 in release mode
    let x255: u8 = "255".parse().unwrap();
    let x0: u8 = "0".parse().unwrap();
    println!("{} + 1 = {}", x255, x255 + 1); // panic in debug mode; 0 in release mode
    println!("{} - 1 = {}", x0, x0 - 1); // panic in debug mode; 255 in release mode
    println!("{} wrapping_add 1 = {}", x255, x255.wrapping_add(1)); // 0
    println!("{} wrapping_sub 1 = {}", x0, x0.wrapping_sub(1)); // 255
    println!("{} saturating_add 1 = {}", x255, x255.saturating_add(1)); // 255
    println!("{} overflowing_add 1 = {:?}", x255, x255.overflowing_add(1)); // (0, true)
    println!("{} checked_add 1 = {:?}", x255, x255.checked_add(1)); // None
    let wrapping_255 = std::num::Wrapping(255u8);
    let wrapping_1 = std::num::Wrapping(1u8);
    let wrapping_0 = std::num::Wrapping(0u8);
    println!("Wrapping({}) + Wrapping(1) = {}", wrapping_255, wrapping_255 + wrapping_1); // 0
    println!("Wrapping({}) - Wrapping(1) = {}", wrapping_0, wrapping_0 - wrapping_1); // 255
}
