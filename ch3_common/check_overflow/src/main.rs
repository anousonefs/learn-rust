#[allow(arithmetic_overflow)]
fn main() {
    let a: u8 = 200;
    let b: u8 = 100;

    println!("a+b = {}", a.wrapping_add(b)); // panic overflow
                                             // println!("a+b = {}", a + b); // panic overflow

    // Using Wrapping addition
    let result_add = a.wrapping_add(b);
    println!("{}", result_add);
    if a > a.wrapping_add(b) {
        println!("Overflow occurred in addition!");
    } else {
        println!("Sum without overflow: {}", result_add);
    }

    // Using Wrapping subtraction
    let result_sub = a.wrapping_sub(b);
    if a < b {
        println!("Overflow occurred in subtraction!");
    } else {
        println!("Difference without overflow: {}", result_sub);
    }

    // Using Wrapping multiplication
    let result_mul = a.wrapping_mul(b);
    println!("{}", result_mul);
    // if a != 0 && b != 0 && a > (u8::MAX / b) {
    //     println!("Overflow occurred in multiplication!");
    // } else {
    //     println!("Product without overflow: {}", result_mul);
    // }
}
