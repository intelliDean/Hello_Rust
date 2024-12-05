
/*
TODO: 1. EACH VALUE IN RUST HAS A VARIABLE THAT IS ITS OWNER
        (let x = 5 => x is the owner of 5)
      2. THERE CAN BE ONLY ONE OWNER AT A TIME
        (let x = 4; let y = x; y is now the only owner of 4)
      3. WHEN THE OWNER GOES OUT OF SCOPE, THE VALUE WILL BE DROPPED
        (when the owner is out of scope, the value is dropped)
*/

pub fn calculate_length(string: &String) -> usize { //&String means that we are passing the reference of the owner
    // EACH VALUE IN RUST HAS A VARIABLE THAT'S ITS OWNER
    string.len()
}

pub fn ownership_change() {
    //THERE CAN BE ONLY ONE OWNER AT A TIME
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); s1 is no longer the owner the "hello" as the owner has been moved to s2

    println!("{}, world!", s2);
}

//WHEN THE OWNER GOES OUT OF SCOPE, THE VALUE WILL BE DROPPED
// outside the block, s2 is out of scope hence the value is dropped, and it cannot be access outside the scope
