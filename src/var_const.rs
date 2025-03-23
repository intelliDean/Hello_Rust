
const GLOBAL: f64 = 24509.8909;
//const is allowed to be declared globally.

pub fn var_mut() {
    println!("----------------- VAR_CONST ---------------------");
    let mut x: i32 = 5;  // the mut keyword enables variable to be mutable
    println!("x = {}", x);

    x += 32;//the keyword 'mut' makes it possible for the x variable to be mutable
    println!("x = {}", x);
    println!("GLOBAL = {}", GLOBAL);
}

pub fn var_as() {

    let mut num: u16 = 234_u8 as u16;
    println!("U8 as U16 = {}", num);
}

pub fn number_sys() {
    println!("----------------- NUMBER SYSTEMS ---------------------");

    let num1 = 1_239; // decimal
    let num2 = 0xff; //hexadecimal
    let num3 = 0o77; //octa
    let num4 = 0b1111_1111; //binary

    println!("{num1} + {num2} + {num3} + {num4} = {}", num1 + num2 + num3 + num4);
}

pub fn constant() {

    let mut  x: String = "Word".to_string();
    const Y: u32 = 3453;
    // You cannot use the keyword 'mut' with const
    // Unlike let, you cannot mutate any variable declared with const

    println!("x = {:?}", x);
    x = "New Word".to_string();

    println!("x = {:?}", x);
    println!("Y = {}", Y);
    println!("GLOBAL = {}", GLOBAL);
}