use demo_library::LivingBeing;
use demo_library::{Dog, Human};

fn main() {
    let mut humans = vec![Human::new("Steve"), Human::new("Bob")];
    for human in &mut humans {
        for _ in 0..20 {
            human.do_birthday();
        }
    }
    humans[0].add_pet(Dog::new("Rex"));
    humans[0].add_pet(Dog::new("Spot"));
    humans[1].add_pet(Dog::new("Albert"));

    for human in &humans {
        println!(
            "{} is {} years old and has {} pets:",
            human.name(),
            human.age(),
            human.pets().len()
        );
        for pet in human.pets() {
            println!("{} makes {} noise", pet.name(), pet.noise());
        }
    }
}
