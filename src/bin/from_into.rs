#![allow(dead_code)]

#[derive(Debug, Clone)]
struct Human{
    name: String,
    age: u32,
}
#[derive(Debug)]
struct Dog{
    name: String,
    age: u32,
}
impl From<Human> for Dog {
    fn from(human: Human) -> Self {
        Dog {
            name: human.name,
            age: human.age,
        }
    }
}
fn main() {
    let human = Human {
        name: String::from("John"),
        age: 30,
    };
    println!("human: {:?}", human);
    let dog = Dog::from(human.clone());
    let dog2: Dog = human.into(); // Into trait implemented for Human automatically
    println!("dog: {:?}", dog);
    println!("dog2: {:?}", dog2);
}
