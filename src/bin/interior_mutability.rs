use std::cell::{Cell, RefCell};

struct Human {
    name: RefCell<String>,
}

struct Pet {
    age: Cell<u8>,
}

fn main() {
    let human = Human {
        name: RefCell::new("John".to_owned()),
    };
    let human_ref = &human;
    let pet = Pet { age: Cell::new(10) };
    let pet_ref = &pet;

    // we can't borrow a value behind a Cell
    let tmp_pet_age = pet_ref.age.get(); // we can get a value
    println!("pet age: {}", tmp_pet_age);
    pet_ref.age.set(tmp_pet_age + 1); // we can set a new value
    println!("pet age: {}", pet_ref.age.get());

    println!("human name: {}", human_ref.name.borrow());
    // we can mutably borrow interior of a RefCell behind an immutable reference
    human_ref.name.borrow_mut().push_str("athan");
    println!("human name: {}", human_ref.name.borrow());
}
