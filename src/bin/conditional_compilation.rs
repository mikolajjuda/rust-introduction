#[cfg(not(feature = "demo_feature"))]
fn some_function() {
    println!("hello");
}

#[cfg(feature = "demo_feature")]
fn some_function() {
    #[cfg(target_os = "linux")]
    println!("linux");
    #[cfg(target_os = "windows")]
    println!("windows");
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    println!("something else");
}

fn main() {
    some_function();
}
