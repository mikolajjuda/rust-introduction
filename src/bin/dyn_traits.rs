struct Human {
    name: String,
}

struct Dog;

trait Speaker {
    fn speak(&self);
}

impl Speaker for Human {
    fn speak(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

impl Speaker for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn main() {
    let v: Vec<Box<dyn Speaker>> = vec![
        Box::new(Human {
            name: "John".to_string(),
        }),
        Box::new(Dog),
    ];
    for s in v.iter() {
        s.speak();
    }
}
