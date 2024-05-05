use my_module::{garden::vegetables::*, person::*, speaking::*};
extern crate restaurant;
use restaurant::eat_at_restaurant;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);

    let person = Person::new(String::from("Bob"), 32);
    person.hello();
    person.speak();

    eat_at_restaurant();
}
