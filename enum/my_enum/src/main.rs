fn main() {
    let result = check_grade(-2);
    match result {
        GradeResult::Error(msg) => println!("{}", msg),
        GradeResult::Value(msg) => println!("{}", msg),
    }

    let result2 = check_grade2(2);
    match result2 {
        Some(msg) => println!("{}", msg),
        None => println!("None"),
    }

    let result3 = check_grade3(2);
    match result3 {
        Err(v) => println!("{}", v),
        Ok(v) => println!("{}", v),
    }

    let y = match check_grade3(3) {
        Err(_) => {
            return;
        }
        Ok(v) => v,
    };
    println!("end.");
    println!("{}", y);

    let z = check_grade3(3).unwrap();
    println!("{}", z);

    let x = check_grade3(-3);
    if let Err(v) = x {
        println!("{}", v);
    } else {
        println!("pass");
    }
}

fn check_grade(score: i32) -> GradeResult {
    if score < 0 || score > 100 {
        return GradeResult::Error("score is not correct".to_string());
    }
    return GradeResult::Value("score is correct".to_string());
}

fn check_grade2(score: i32) -> Option<String> {
    if score < 0 || score > 100 {
        return None;
    }
    return Some("A".to_string());
}

fn check_grade3(score: i32) -> Result<String, String> {
    if score < 0 || score > 100 {
        return Err("score is not correct".to_string());
    }
    Ok("A".to_string())
}

#[derive(Debug)]
enum GradeResult {
    Error(String),
    Value(String),
}
