
//TODO: THERE ARE JUST 4 SCALAR (PRIMITIVE DATA TYPES IN RUST)

pub fn integers() {
    println!("================ INTEGER ======================");
    let x: i8 = -50;
    /*
    i8 mean signed integer and it means 2 things.
        1. it can take both positive and negative values.
        2. the 8 means it can take 8bits and the range is: (2^8 - 1 )
    */

    let y: u32 = 9000;
    /*
        u32 mean unsigned integer meaning it can only take positive values
        and also take 32 bits and the range is: (2^32 - 1)
    */

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    println!("======================================");

    let max_32: i32 = 2147483647;
    let max_64: i64 = 9223372036854775807;

    println!("The value of i32 is: {}", max_32);
    println!("The value of i64 is: {}", max_64);
}

pub fn float() {
    println!("================== FLOAT ====================");
    //Floats are numbers with decimals
    //There are just 2 of them: f32 and f64

    let pi: f64 = 3.14;

    println!("The value of pi is: {}", pi);
}

pub fn boolean() {
    println!("================== BOOLEAN ====================");
    //These are either true or false

    let is_reg: bool = false;
    let res: &str;
    if is_reg {
        res = "Yes";
    } else {
        res = "No";
    }

    println!("Has he registered: {}", res);
}

pub fn character() {
    println!("================ CHARACTERS ======================");
    //These are characters of the alphabets

    let letter = 'Z';

    println!("The letter is: {}", letter);
    println!("======================================");
}