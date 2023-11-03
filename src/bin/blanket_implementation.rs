trait Square<T> {
    fn square(self) -> T;
}

impl<T> Square<T> for T
where
    T: std::ops::Mul<Output = T> + Copy,
{
    fn square(self) -> T {
        self * self
    }
}

fn main() {
    println!("{}", 2u8.square());
    println!("{}", 16.5f32.square());
}
