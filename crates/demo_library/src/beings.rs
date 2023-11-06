use std::fmt::Debug;

mod dog;
mod cat;
mod human;

pub use dog::Dog;
pub use cat::Cat;
pub use human::Human;

pub trait LivingBeing: Debug + dyn_clone::DynClone {
    fn name(&self) -> &str;
    fn age(&self) -> u8;
    fn do_birthday(&mut self);
    fn rename(&mut self, new_name: &str);
}
dyn_clone::clone_trait_object!(LivingBeing);

pub trait Pet: LivingBeing {
    fn is_good_boy(&self) -> bool {
        true
    }
}
dyn_clone::clone_trait_object!(Pet);
