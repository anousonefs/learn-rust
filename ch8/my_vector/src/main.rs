fn main() {
    // let mut v = Vec::new();
    // v.push(4);
    // v.push(12);
    // v.push(14);
    // v.push(16);
    // v.push(20);
    // let v = vec![1, 2, 3, 4, 5];
    // let third: i32 = v[0];
    // println!("The third element is {third}");
    //
    // let third: Option<&i32> = v.get(4);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // };

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");

    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{i}");
    // }

    // let mut v = vec![100, 32, 57];
    // println!("{:?}", v);
    // for i in &mut v {
    //     *i += 50;
    // }
    // println!("{:?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}
