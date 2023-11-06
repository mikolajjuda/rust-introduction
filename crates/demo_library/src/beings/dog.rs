use super::LivingBeing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dog{
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

#[cfg(test)]
mod tests {
    use super::*;
}
