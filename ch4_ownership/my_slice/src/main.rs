fn main() {
    let mut s = String::from("hello world");

    let word = first_word2(&s);
    println!("the first word is: {}", word);

    s.clear();

    // let mut s = String::from("hello world");
    //
    // let word = first_word1(&s); // word will get the value 5
    //
    // s.clear(); // this empties the String, making it equal to ""
    //
    // println!("the first word is: {}", word);
}

#[allow(dead_code)]
fn first_word1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

#[allow(dead_code)]
fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
