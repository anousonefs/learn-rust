fn main() {
    println!("Hello, world!");
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("anousone@gmail.com"),
        ..user1
    };

    println!("{:?}", user1.active);
    // println!("{:?}", user1.username); // error
    println!("{:?}", user1.sign_in_count);
    println!("{:?}", user2);
}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
