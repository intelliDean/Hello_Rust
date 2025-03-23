pub fn age_condition(age: u8) {
    println!("-------------- CONTROL FLOW ----------------");
    if age == 0 {
        eprintln!("Age cannot be 0");
    } else if age > 0 && age < 18 {
        println!("You are a child");
    } else if age > 18 && age < 30 {
        println!("You are an adult");
    } else {
        println!("You are a parent");
    }
}

pub fn if_return(condition: bool) {
    let res = if condition {
        "It is true!"   //The return type here must match the one in the else statement so that the variable accepting does not have inconsistent type 
    } else {
        "It is false"
    };

    println!("{res}");
}
