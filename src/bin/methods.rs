struct Human {
    name: String,
    age: u32,
}

impl Human {
    // associated function
    fn age_difference(human1: &Self, human2: &Self) -> i64 {
        human1.age as i64 - human2.age as i64
    }

    // method (methods are also associated functions)
    fn say_name(&self) { // &self as a first parameter is an alias for self: &Self
        println!("My name is {}", self.name);
    }

    // &mut self as a first parameter is an alias for self: &mut Self
    fn birthday(&mut self) {
        self.age += 1;
    }

    fn die(self) {
        // self as a first parameter is an alias for self: Self
        println!("{} dies", self.name);
    }
}

fn main() {
    let mut human = Human {
        name: String::from("John"),
        age: 30,
    };

    human.say_name(); // reference to human is automatically created
    human.birthday();
    println!("{} is {} years old", human.name, human.age);
    human.die(); // human is moved to the method and is not valid anymore

    let human1 = Human {
        name: String::from("Steve"),
        age: 2,
    };
    let human2 = Human {
        name: String::from("John"),
        age: 123,
    };
    println!(
        "age difference is {}",
        Human::age_difference(&human1, &human2)
    );
}
