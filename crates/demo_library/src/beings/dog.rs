use super::{LivingBeing, Pet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dog {
    name: String,
    age: u8,
}

impl Dog {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            age: 0,
        }
    }
}

impl LivingBeing for Dog {
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

impl Pet for Dog {
    fn noise(&self) -> &str {
        "woof"
    }
}

impl From<super::Cat> for Dog {
    fn from(cat: super::Cat) -> Self {
        Self {
            name: cat.name().to_string(),
            age: cat.age(),
        }
    }
}

impl<D: std::ops::Deref<Target = super::Cat>> From<D> for Dog {
    fn from(cat: D) -> Self {
        Self {
            name: cat.name().to_string(),
            age: cat.age(),
        }
    }
}
