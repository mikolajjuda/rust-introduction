fn duplicate<T: Clone>(x: &T) -> (T, T) {
    (x.clone(), x.clone())
}

// syntactic sugar for
// fn generic_print<T: std::fmt::Display + ?Sized>(x: &T) {
fn generic_print(x: &(impl std::fmt::Display + ?Sized)) {
    println!("generically printed: {}", x);
}

fn square<T>(x: T) -> T
where
    T: std::ops::Mul<Output = T> + Copy,
{
    x * x
}

fn main() {
    let s = "hello".to_string();
    let (s1, s2) = duplicate(&s);
    println!("s1: {}, s2: {}", s1, s2);
    generic_print(&123);
    generic_print("hello");
    println!("square(8): {}", square(8i8));
    println!("square(2.5): {}", square(2.5f64));
}
