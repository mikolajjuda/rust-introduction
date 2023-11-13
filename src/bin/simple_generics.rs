struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<String> {
    fn special_x(&self) -> String {
        format!("idk why Strings but here you go: {}", self.x)
    }
}

fn main() {
    let p1: Point<i16> = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.8, y: 10.2 };
    let p3 = Point {
        x: "hello".to_owned(),
        y: "world".to_owned(),
    };
    println!("p1.x(): {}, p2.y(): {}, p3.x(): {}", p1.x(), p2.y(), p3.x());
    println!("p3.special_x(): {}", p3.special_x());
}
