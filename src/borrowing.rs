
/* References: enables you to borrow values without taking ownership of the value.
    References can either be mutable and immutable.
    To create a reference, you add '&' before the variable.
*/

pub fn reference() {
    let x: i32 = 2030;
    let y: i32 = x;

    println!("value of x: {}", x); //X is moved yet it's still being used. I guess because it's primitive
    println!("value of y: {}", y);
}

pub fn ref_string() {
    println!("------------------- BORROWING ------------------------");
    let word: String = String::from("Amsterdam");
    let new_word: &str = &word; //For this reference to work, & needs to be added to variable word and the new_word type will have to change to &str to show it's a reference

    println!("value of word: {}", word); //The value of string (a scalar type) cannot be used after being moved
    println!("value of new_word: {}", new_word);
}

pub fn ref_mut_string() {
    let mut city: String = String::from("FRANCESCO");
    let new_city = &mut city;

    print!("value of city from the borrower: {}\n", new_city); // \n gives it a new line (learnt that from java)

    *new_city = "CALIFORNIA".to_string(); // This mutates the variable but this also affects the owner

    println!("value of the owner after the borrower was updated: {}", city);

    // TODO: YOU CAN ONLY HAVE ONE MUTABLE REFERENCE BUT MANY IMMUTABLE REFERENCES
}