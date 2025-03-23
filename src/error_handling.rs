/*
enum Option<T> { // Defines the generic Option type
    Some(T),    // Represents  a value
    None,       // Represents no value
}

enum Result<T, E> { // Define the generic Result type
    Ok(T),          // Represents a value
    Err(E),         // Represents an error
}
*/

/* TODO: The Option is used to cater for for error in the sense that if the result is not bringing
    the expected result we can use None to receive the result and we use Some to receive if the
    result is expected
*/
pub fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    println!("-------------------- ERROR HANDLING ------------------------");
    if denominator == 0.0 {
        // Unexpected result
        None
    } else {
        Some(numerator / denominator) // Expected result
    }
}

pub fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Denominator cannot be zero"))
    } else {
        Ok(numerator / denominator)
    }
}
