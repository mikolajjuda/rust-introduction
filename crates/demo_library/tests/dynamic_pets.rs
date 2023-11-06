use demo_library::{Cat, Dog, Human};

#[test]
fn dynamic_pets() {
    let mut human = Human::new("Jason");
    human.add_pet(Cat::new("Mittens"));
    human.add_pet(Dog::new("Steve"));
    human.add_pet(Cat::new("Pierre"));
    human.add_pet(Dog::new("Elisabeth"));

    let correct_sounds = ["meow", "woof", "meow", "woof"];
    for (s, &c) in human.pets().iter().zip(correct_sounds.iter()) {
        assert_eq!(s.noise(), c);
    }
}
