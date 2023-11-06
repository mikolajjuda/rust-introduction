use super::LivingBeing;
use super::Pet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cat {
    name: String,
    age: u8,
    remaining_lives: u8,
    pub scratched_owner: bool,
}

impl Cat {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            age: 0,
            remaining_lives: 9,
            scratched_owner: false,
        }
    }

    pub fn remaining_lives(&self) -> u8 {
        self.remaining_lives
    }

    pub fn die(&mut self) {
        self.remaining_lives = self.remaining_lives.saturating_sub(1);
    }
}

impl LivingBeing for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u8 {
        self.age
    }

    fn do_birthday(&mut self) {
        self.age += 1;
    }

    fn rename(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}

impl Pet for Cat {
    fn is_good_boy(&self) -> bool {
        !self.scratched_owner
    }
}

impl From<super::Dog> for Cat {
    fn from(dog: super::Dog) -> Self {
        Self {
            name: dog.name().to_string(),
            age: dog.age(),
            remaining_lives: 1,
            scratched_owner: false,
        }
    }
}

impl<D: std::ops::Deref<Target = super::Dog>> From<D> for Cat {
    fn from(dog: D) -> Self {
        Self {
            name: dog.name().to_string(),
            age: dog.age(),
            remaining_lives: 1,
            scratched_owner: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cat_from_dog() {
        let dog = crate::Dog::new("Steve");
        let cat = Cat::from(dog);
        assert_eq!(
            cat,
            Cat {
                name: "Steve".to_string(),
                age: 0,
                remaining_lives: 1,
                scratched_owner: false,
            }
        );
    }

    #[test]
    fn cat_from_dog_ref() {
        let dog = crate::Dog::new("Steve");
        let cat = Cat::from(&dog);
        assert_eq!(
            cat,
            Cat {
                name: "Steve".to_string(),
                age: 0,
                remaining_lives: 1,
                scratched_owner: false,
            }
        );
    }
}
