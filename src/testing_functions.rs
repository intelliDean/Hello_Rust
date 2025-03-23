pub fn floating_point_precision() {
    println!("------------ FLOATING POINT PRECISION -------------");

    // assert_eq!(0.1 + 0.2, 0.3); // 0.30000000000000004
    //The default floating point is f64 which is very precise hence 0.1 + 0.2 is not 0.3 but 0.30000000000000004
    // to make it work like that we need a less precise type which is f32

    assert_eq!(0.1_f32 + 0.2_f32, 0.3_f32);
    // assert_eq!(0.1 as f32 + 0.2 as f32, 0.3 as f32); // this also works

    println!("Success!")
}

pub fn for_loop_types() {
    println!("------------ FOR LOOP TYPES -------------");
    //for loop 1
    println!("exclusive");
    for i in 0..5 {
        // in this instance, it will iterate from 0 to 4 with 5 excluded
        print!("{} ", i);
    }
    println!();

    println!("inclusive");
    for i in 'a'..='z' {
        //in this instance, 5 will be included because of the '=' sign so it will iterate from 0 to 5
        print!("{} ", i as u16);
    }
}

use std::cmp::PartialEq;
use std::ops::{Range, RangeInclusive};
pub fn for_loop_range() {
    println!("------------ FOR LOOP RANGE -------------");

    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

/* TODO: BASIC TYPE AND THEIR SIZES IN MEMORY
    CHAR: Single character of size 4 bytes
    BOOL: Boolean value of true  or false of size 1 byte
    UNIT: Empty tuple of size 0 bytes, used to return 'nothing' in expressions or functions
*/

use std::mem::size_of_val;
pub fn size_in_memory() {
    println!("------------ SIZE IN MEMORY -------------");
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4); // 4 bytes;

    let c2 = 42;
    assert_eq!(size_of_val(&c2), 4); //4 bytes

    println!("Success!");
}

//TODO: unit type is a type that does not hold any value. It's an empty tuple with a 0 byte size
// It is returned when a function returns nothing, though you do not have to annotate that, it is implicitly done by Rust.

pub fn return_unit_type() -> () {
    println!("------------ RETURN UNIT TYPE -------------");

    let ret = ();

    assert_eq!(ret, inner());

    println!("Success!");

    fn inner() {
        println!("THIS FUNCTION RETURNS NOTHING");
    }
}

pub fn turn_statement_to_expressions() {
    println!("------------ TURN STATEMENT_TO EXPRESS -------------");
    let v = {
        let mut x = 1;
        x = x * 2; // this is a statement because it is assigning
        x //this returns x to v, this makes this block of code an expression and not statement
    };

    assert_eq!(v, 2);

    println!("Success!\n v = {}", v)
}

pub fn sum_destructured_tuples() {
    println!("---------------- DESTRUCTURED TUPLE ------------------");
    let (x, y, z) = (1, 2, 3);

    let s: i32 = sum(x, y, z);

    assert_eq!(s, 6);

    println!("Success!\n s = {}", s);
    fn sum(x: i32, y: i32, z: i32) -> i32 {
        x + y + z
    }
}

// pub fn diverging_functions(num: u8) -> !{
//     /* TODO: Diverging functions never return to the caller,
//              so they may be used in places where a value of any type is expected.
//     */
//
//     match num {
//         1 => {
//             println!("It's as expected!, {}", 1)
//         }
//         _ => {
//             println!("It's not as expected!, {}", num);
//         }
//     };
//
//     panic!()
// }

pub fn dereferencing() {
    println!("-------------- DEREFERENCING -------------------");

    let x: Box<i32> = Box::new(100);
    // by default, primitive data types are stored on the stack because they have
    // fixed size and their size is known at compiled time, but you can use Box<i32>
    // to put them in the heap memory

    let mut y: Box<i32> = Box::new(200);

    *y = 201;
    // Forget not that 'y' is not a direct i32, but holds a reference to the address
    // of the value on heap so assigning an i32 would not work. To assign an i32 to it
    // you need to dereference it using the '*' next to the variable name.
    // This directly goes to the address and get the value there.

    assert_eq!(*x, 100);
    assert_eq!(*y, 201);

    println!("Success!");
}

pub fn move_and_reference() {
    println!("-------------- MOVE AND REFERENCE ---------------");

    #[derive(Debug)]
    struct User {
        username: String,
        age: Box<u8>,
    }

    let user1: User = User {
        username: String::from("dean"),
        age: Box::new(12),
    };

    let User {
        username, //from the user1, username was completely moved; hence user1 is no longer the owner of username
        ref age, // from the user1, age was referenced and not moved; hence user1 is still the owner of age
    } = user1;

    println!("Username: {}, age: {}", username, age);

    println!("The user1 struct: {:?}", user1.age);

    // println!("The user1 struct : {:?}", user1.username); this cannot be accessed as it was moved
}

pub fn borrowing() {
    println!("-------------- BORROWING -------------");

    let mut str1 = String::from("Hello");

    let r1 = &str1;
    let r2 = &str1;
    println!("r1: {}, r2: {}", r1, r2);
    // this works because after this line r1 and r2 will no longer be used in this scope

    let r3 = &mut str1;
    println!("r3: {}", r3); // then r3 can also be used after r1 and r2 are dropped

    // println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
    // trying to access all here is the problem because the first rule of
    // borrowing states that we can either have any number of immutable
    // references or a single mutable reference
}

pub fn dangling_reference() {
    println!("-------------- DANGLING REFERENCE -------------");

    let reference_to_nothing: /*&String*/ String = dangling_reference();
    println!("Success!");

    fn dangling_reference() -> String {
        let s: String = String::from("Hello"); // 's' will go out of scope at the end
        // of this function scope. It will be dropped, passing its reference means the
        // reference will be pointing at nothing as the owner has been dropped, so moving 's' is the correct way as it will be moved and ownership passed. In this way, this obeyed the borrowing second rule which states that references must always be valid

        // &s  // passing reference to 's' is against the borrowing rule; hence we pass 's' instead of its reference
        s
    }
}

pub fn memory_location() {
    println!("-------------- MEMORY LOCATION -------------");

    let x: i32 = 2;
    let y: &i32 = &x;

    println!("x memory location: {:p}", y); // {:p} is used to get the memory location of a variable
}

pub fn reference_value() {
    println!("-------------- REFERENCE VALUE -------------");

    let x: i32 = 100;
    let y: &i32 = &x;

    println!("x = {}, y = {}", x, y); // don't let this to fool you, y is not holding
    // the value of x, it's only holding the pointer (address) to the location of the
    // value of x in memory. To really get the actual value of where y points to, '*"
    // needs to be added to y like this '*y"
    assert_eq!(x, *y);
    println!("Success!");
}

pub fn using_ref_instead_of_ampersand() {
    println!("-------------- USING REF INSTEAD OF & -------------");
    let c: char = 'A';

    let r1: &char = &c;
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    assert_eq!(get_address(r1), get_address(r2));

    println!("r1 address: {}, r2 address {}", get_address(r1), get_address(r2));

    fn get_address(r: &char) -> String {
        format!("{:p}", r)
    }
}

pub fn str() {
    println!("------- EMPTY STRING -------");

    let mut s: String = String::new();
    // Only String is mutable, string literal and string slice are just a read-only; hence they are immutable

    s.push_str("Hello world");
    s.push('!');
    println!("{}", s);
}

pub fn replace_func() {
    println!("-------------- REPLACE FUNC -------------");

    let mut s: String = String::from("I love coding");
    let s1 = s.replace("coding", "making money");

    assert_eq!(s1, "I love making money");

    println!("s1: {}", s1);
}

pub fn plenty_array_elements() {
    println!("------------- PLENTY ARRAY ELEMENTS -------------");

    let list: [i32; 100] = [1; 100]; // instead of filling the whole array with 1 hundred times like this [1,1,1,1,1,.1], we can do it like this: [1; 100] as long as it's the same number

    assert_eq!(list[0], 1);
    assert_eq!(list.len(), 100);

    println!("Index 0 element: {}", list.get(0).unwrap()); //the get() is safer than the list[] because get return Option type

    println!("Success!");
}

pub fn tup() {
    println!("------------- TUPLE -------------");

    let tup: (&str, (u32, String), i32) = ("Hello", (34, "World".to_string()), 32);

    assert_eq!(tup.1.1, "World".to_string());

    println!("Success!");
}

pub fn print_struct() {
    #[derive(Debug)] //todo: #[derive(Debug)]  makes the struct printable
    struct Person {
        name: String,
        age: u8,
    }

    let p: Person = Person {
        name: String::from("Dean"),
        age: 12,
    };

    let _name: String = p.name.clone();

    println!("{}, {}, {:?}", _name, p.age, p);
}

pub fn enum_types() {
    println!("------------- ENUM TYPES -------------");

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 }, // struct-like variant
        Write(String), //tuple-like variant
        ChangeColor(i32, i32, i32), // tuple-like variant
    }

    let msg1: Message = Message::Move { x: 10, y: 20 }; //this is of type Message and not of type struct
    let msg2: Message = Message::Write(String::from("hello")); //this is of type Message and not of type String

    println!("{:?}", msg1);
}

pub fn match_pattern() {
    println!("------------- MATCH PATTERN -------------");
    enum Direction {
        EAST,
        NORTH,
        WEST,
        SOUTH,
    }
    ;

    let dir: Direction = Direction::SOUTH;

    match dir {
        Direction::EAST => println!("EAST"),
        Direction::WEST => println!("WEST"),
        Direction::NORTH | Direction::SOUTH => println!("NORTH or SOUTH"),
    }
}

pub fn match_bool() {
    println!("------------- MATCH BOOL -------------");
    let res: bool = true;

    let result: u8 = match res {
        true => 1,
        false => 0,
    };

    assert_eq!(result, 1);
    println!("Success!");
}

pub fn match_msg() {
    println!("------------- MATCH MSG -------------");
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::ChangeColor(255, 255, 0),
    ];

    for m in msg {
        match m {
            Message::Move { x: a, y: b } => {
                assert_eq!(a, 10);
                assert_eq!(b, 20);
            }
            Message::ChangeColor(r, g, b) => {
                assert_eq!(r, 255);
                assert_eq!(b, 0);
            }
            _ => println!("No data in this variant")
        }
    }
}

pub fn matches_not_match() {
    println!("------------- MATCHES NOT MATCH -------------");
    let alphabets: [char; 7] = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    for alph in alphabets {
        assert!(matches!(alph, '0'..='9' | 'A'..='Z' | 'a'..='z'));
    }

    println!("Success!");
}



pub fn matches() {
    enum MyEnum {
        FOO,
        BAR
    }

    let mut count: u8 = 0;
    let v: Vec<MyEnum> = vec![MyEnum::FOO, MyEnum::BAR, MyEnum::FOO];

    for e in v {
        // if e == MyEnum::FOO {} THIS WILL NOT WORK
        if matches!(e, MyEnum::FOO) {
            count += 1;
        }
    }

    assert_eq!(count, 2);
    println!("Success!");
}

pub fn if_let() {
    println!("------------- IF LET -------------");

    let o: Option<i32> = Some(23);

    if let Some(i) = o {
        println!("{}", i);
        println!("Success!");
    }
}

pub fn dest_some() {
    let age: Option<i32> = Some(23);

    if let Some(_age) = age {
        assert_eq!(_age, 23);
    }

    match age {
        Some(age) => println!("{}", age),
        _ => ()
    }
}

// pub fn () {
//     println!("------------- PAT -------------");
//
//     let n: u8 = 4;
//
//     match n {
//         1 => println!("1"),
//         2 | 3 | 4 | 5 => println!("2 to 5"),
//         6 .. 10 => println!("6 to 10"),
//         _ => println!("not implemented yet")
//     }
// }