use demo_library::{Cat, Dog};

#[test]
fn dog_from_cat() {
    let cat = Cat::new("Mittens");
    let dog = Dog::from(cat);
    assert_eq!(dog, Dog::new("Mittens"));
}

#[test]
fn dog_from_cat_ref() {
    let cat = Cat::new("Mittens");
    let dog = Dog::from(&cat);
    assert_eq!(dog, Dog::new("Mittens"));
}

fn leave_cat_barely_alive(cat: &mut Cat) {
    while cat.remaining_lives() > 1 {
        cat.die();
    }
}

#[test]
fn cat_from_dog() {
    let dog = Dog::new("Steve");
    let cat = Cat::from(dog);
    let mut correct_cat = Cat::new("Steve");
    leave_cat_barely_alive(&mut correct_cat);
    assert_eq!(cat, correct_cat);
}

#[test]
fn cat_from_dog_ref() {
    let dog = Dog::new("Steve");
    let cat = Cat::from(&dog);
    let mut correct_cat = Cat::new("Steve");
    leave_cat_barely_alive(&mut correct_cat);
    assert_eq!(cat, correct_cat);
}
