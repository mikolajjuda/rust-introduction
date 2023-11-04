use std::cell::{Cell, RefCell};
use std::rc::Rc;

struct Human {
    name: RefCell<String>,
}

struct Pet {
    age: Cell<u8>,
    owner: Rc<Human>,
}

fn main() {
    let human = Rc::new(Human {
        name: RefCell::new("John".to_string()),
    });
    let pet = Pet {
        age: Cell::new(10),
        owner: human.clone(), // cloning a reference
    };
    std::mem::drop(human); // dropping a reference
    println!("pet owner name: {}", pet.owner.name.borrow());
    // we can mutably borrow interior of a RefCell behind an immutable reference
    pet.owner.name.borrow_mut().push_str("athan");
    println!("pet owner name: {}", pet.owner.name.borrow());

    // we can't borrow a value behind a Cell
    let tmp_pet_age = pet.age.get(); // we can get a value
    println!("pet age: {}", tmp_pet_age);
    pet.age.set(tmp_pet_age + 1); // we can set a new value
    println!("pet age: {}", pet.age.get());
}
