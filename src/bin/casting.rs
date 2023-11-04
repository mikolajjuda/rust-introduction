fn main() {
    let decimal = 65.99999f64;
    let integer = decimal as u8;
    let character = integer as char;
    println!("{} -> {} -> {}", decimal, integer, character);
    println!("-1i8 as u8 {}", -1i8 as u8);
    println!("1000u32 as u8 {}", 1000u32 as u8);
    println!(" 232i32 as i8 {}", 232i32 as i8);
    println!("-10023f32 as u8 {}", -1023f32 as i8);
    println!("NAN f32 as u8 {}", f32::NAN as u8);
}