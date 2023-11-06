use super::LivingBeing;
use super::Pet;

#[derive(Debug, Clone)]
pub struct Human {
    name: String,
    age: u8,
    pets: Vec<Box<dyn Pet>>,
}

impl Human {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            age: 0,
            pets: Vec::new(),
        }
    }

    pub fn pets(&self) -> &Vec<Box<dyn Pet>> {
        &self.pets
    }

    pub fn pets_mut(&mut self) -> &mut Vec<Box<dyn Pet>> {
        &mut self.pets
    }
}

impl LivingBeing for Human {
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

#[cfg(test)]
mod tests {
    use super::*;
}
