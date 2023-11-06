//! Example library demonstrating some Rust features related to productive development.
//! 
//! # Examples
//! ```
//! use demo_library::{Human, LivingBeing};
//! 
//! let mut human = Human::new("Steve");
//! human.do_birthday();
//! human.rename("Michael");
//! assert_eq!(human.name(), "Michael");
//! assert_eq!(human.age(), 1);
//! ```
//! 
//! ```
//! use demo_library::{Cat, LivingBeing};
//! 
//! let mut cat = Cat::new("Mittens");
//! cat.do_birthday();
//! cat.die();
//! assert_eq!(cat.name(), "Mittens");
//! assert_eq!(cat.age(), 1);
//! assert_eq!(cat.remaining_lives(), 8);
//! ```
mod beings;

pub use beings::Cat;
pub use beings::Dog;
pub use beings::Human;

pub use beings::LivingBeing;
pub use beings::Pet;
