use std::arch::asm;

unsafe fn dangerous_square(x: u32) -> u64 {
    let hi: u32;
    let lo: u32;
    asm!("mul eax", inlateout("eax") x => lo, lateout("edx") hi);
    ((hi as u64) << 32) | (lo as u64)
}

fn main() {
    let a = 1000000;
    let b = unsafe { dangerous_square(a) };
    println!("{a}^2 = {b}");

    let arr = [1, 2, 3, 4, 5];
    let p: *const i32 = &arr[0];
    let x = unsafe { *p.offset(3) };
    println!("x = {}", x);
}
