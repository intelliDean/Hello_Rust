/* TODO:
    Vector (Can grow and shrink dynamically),
    UTF8 ,
    HashMaps
*/
use std::collections::HashMap;
use std::fmt::format;

pub fn vector() {
    println!("------------------- COLLECTIONS ----------------------");
    let mut vector1: Vec<String> = Vec::new();

    let vector2: Vec<u32> = vec![1, 2, 3, 4, 5];

    vector1.push(String::from("Hello"));
    vector1.push(String::from("From"));
    vector1.push(String::from("The"));
    vector1.push(String::from("Other"));
    vector1.push(String::from("Side"));

    for i in 0..vector1.len() {
        println!("{}: {}", vector2[i], vector1[i]);
    }
}

pub fn init_vector() {
    println!("------------------- INIT VECTORS --------------------");

    let mut vector: Vec<i32> = Vec::new();

    vector = vec![1, 2, 3, 4];
    vector.push(5);
    vector.push(6);
    vector.push(7);
    vector.push(8);
    vector.push(9);
    vector.push(10);

    for v in &vector {
        println!("{v}")
    }
}

pub fn retrieve_vector() {
    println!("------------------- RETRIEVE VECTORS -------------------");

    let vector = vec![1, 2, 3, 4];

    println!("Element at index 2 is {}", &vector[2]); // getting the reference

    let third_element: Option<&i32> = vector.get(3);

    match third_element {
        Some(third) => println!("third element is {}", third),
        None => println!("Third element not found"),
    }
}

pub fn uft8() {
    println!("--------------- UFT8 STRING --------------------");

    // Different ways to declare and instantiate a string in Rust

    let s1: String = String::from("First String ");

    let s2: String = "Second String ".to_string();

    let mut s3: String = String::from("Mutable ");

    s3.push_str("String"); // To push a string into another string, push_str is used
    s3.push('!'); // the push function is used to push just a character at a time while push_str is used to push a group of character (string) at a time.
                  // Notice that the character takes single quotation unlike the string that takes double quotation

    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);

    let s4: String = s1 + &s2 + &s3; // To concatenate, we need to add 1 owned and others borrowed. You cannot add 2 owned, and you cannot add 2 borrowed.
                                     // so the first string will have to be moved and the ownership moved while the rest will be borrowed.
                                     // the function that does the concatenation looks like this under the hood => fn add(self, s1: &str, s2: &str) -> String {} where the self is the owned and the rest are the borrowed

    println!("Concatenate: s4: {}", s4);


    let s5 = format!("{s2}-{s3}");
    println!("Format: s5: {}", s5);

}

pub fn hash_map() {
    println!("--------------- HASHMAP --------------------------");

    let mut balance_of = HashMap::new();
    balance_of.insert(String::from("Dean"), 10000);
    balance_of.insert(String::from("Michael"), 30000);

    let name: String = String::from("Dean");

    let score = balance_of.get(&name).copied().unwrap_or(0);
    println!("{name} balance: {}", score);

    for (key, value) in &balance_of {
        println!("{key}: {value}");
    }
}
