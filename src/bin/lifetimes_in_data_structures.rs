struct Human {
    name: String,
}

struct Pet<'a> {
    name: String,
    owner: &'a Human,
}

fn main() {
    let human = Human {
        name: "John".to_owned(),
    };
    let pet = Pet {
        name: "Steve".to_owned(),
        owner: &human,
    };
    println!("{}'s owner name is {}", pet.name, pet.owner.name);
}
