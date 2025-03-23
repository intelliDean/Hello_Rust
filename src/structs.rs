#[derive(Debug)] //I needed to add this so that i could access what was returned in the main fn
pub(crate) struct User {
    pub username: String,
    pub email: String,
    pub age: u8,
    pub active: bool,
}

#[derive(Debug)]
pub struct Man(String, u32, bool);

pub fn struct1() {
    println!("------------------- STRUCT --------------------------");
    let mut dean: User = User {
        username: "Dean".to_string(),
        email: "dean@gmail.com".to_string(),
        age: 12,
        active: true,
    };

    println!("Email: {}", dean.email);

    dean.email = String::from("michael_dean@gmail.com");

    println!("Username: {}", dean.username);
    println!("Updated Email: {}", dean.email);
}


pub fn struct2() {

    let mut dean: User = User {
        username: "Dean".to_string(),
        email: "dean@gmail.com".to_string(),
        age: 12,
        active: true,
    };

    let mut user2 = User {
        username: String::from("James21x"),
        ..dean //this means this new struct is using all the information of the dean struct except the username that is explicitly stated
    };

    println!("user2 username: {}", user2.username);
    println!("user2 email: {}", user2.email);
}

pub fn struct_return() -> User {
    User {
        username: String::from("Tevuz"),
        email: String::from("email@gmail.com"),
        age: 23,
        active: true,
    }
}

pub fn tuple_struct() -> Man {

    let origin = Man(String::from("Michael"), 12, true);

    origin
}

pub fn unit_like_struct() {
    struct AlwaysEqual; 
}