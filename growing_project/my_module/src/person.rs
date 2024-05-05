use crate::speaking::Speaking;

#[allow(dead_code)]
pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
    pub fn hello(&self) {
        println!("Hello, my name is {}", self.name)
    }
}

impl Speaking for Person {
    fn speak(&self) {
        println!("I'm {}", self.name)
    }
}
