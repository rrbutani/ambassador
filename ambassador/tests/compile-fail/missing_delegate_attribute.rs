extern crate ambassador;

use ambassador::{delegatable_trait, Delegate};

#[delegatable_trait]
pub trait Shout {
    fn shout(&self, input: &str) -> String;
}

pub struct Cat;

impl Shout for Cat {
    fn shout(&self, input: &str) -> String {
        format!("{} - meow!", input)
    }
}

pub struct Dog;

impl Shout for Dog {
    fn shout(&self, input: &str) -> String {
        format!("{} - wuff!", input)
    }
}

#[derive(Delegate)] //~ ERROR proc-macro derive panicked
pub enum Animals {
    Cat(Cat),
    Dog(Dog),
}

pub fn main() {
    let foo_animal = Animals::Cat(Cat);
    println!("{}", foo_animal.shout("BAR")); //~ ERROR no method named `shout` found for enum `Animals` in the current scope
}
