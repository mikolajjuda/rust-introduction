struct Dog;
struct Cat;
struct Human {
    name: String,
}

trait Speaker {
    fn speak(&self); // no default implementation
    //default implementation
    fn is_alive(&self) -> bool {
        true
    }
}

impl Speaker for Dog {
    fn speak(&self) {
        println!("woof!");
    }
}

impl Speaker for Cat {
    fn speak(&self) {
        println!("meow!");
    }
}

impl Speaker for Human {
    fn speak(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    let person = Human {
        name: "John".to_string(),
    };
    dog.speak();
    cat.speak();
    person.speak();
    println!("Is the cat alive? {}", cat.is_alive());
}
