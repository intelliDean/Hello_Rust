
pub fn add(num1: u32, num2: u32) -> u32 {
    println!("============== FUNCTIONS ================");
    num1 + num2 //Notice that it does not have a semicolon at the end, this is because it is returning a value and the return keyword is not needed
}


pub fn my_info(name: &str, age: u8, email: String, height: f32) {
    println!("
        my name is {}
        I am {} years old
        email: {}
        height: {} cm
    ", name, age, email, height);
}

pub fn block() {

    let res = {
        let price: u32 = 34;
        let quantity: u32 = 200;

        price * quantity
    };

    println!("Result is {}", res);
}

pub fn check_add(num1: i32, num2: i32) {
    println!("------------- CHECKED ADD -------------");

    let res1 = num1 + num2;

    println!("Plain result is {}", res1);

    let res2 = i32::checked_add(num1, num2).unwrap(); //This does the same thing as

    println!("Checked result is {}", res2);
}

pub fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg /  (height_m * height_m)
}

/*
TODO: EXPRESSION is anything that returns a value.
      Examples:
      number like 5
      boolean like true or false
      function that returns value like: add(3, 5);
      conditional flow like if statement
      blocks like anything in { code  }
      .
      STATEMENT is anything that does not return a value
      Examples:
      variable declaration: let y = let x
      function definition: fn food() {}

*/

