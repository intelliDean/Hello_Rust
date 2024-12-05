//TODO: ARRAYS, TUPLES, SLICES, STRINGS (SLICE STRING)

//ARRAYS: are fixed size collection of elements of the same type (homogenous types)

pub fn arrays() {
    println!("================= ARRAYS =====================");
    let num_array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Numbers: {:?}", num_array);

    //TODO: I need to understand the debugger and display formatter

    let fruits: [&str; 3] = ["Apple", "Orange", "Banana"];
    println!("All Fruits: {:?}", fruits);
    println!("Fruit 1: {:?}", fruits[0]);
    println!("Fruit 2: {}", fruits[1]);
    println!("Fruit 3: {}", fruits[2]);
}

//TUPLES: contains heterogeneous (elements of different type) collections of fixed size.
pub fn tuples() {
    println!("================ TUPLES ================");

    let human: (&str, u8, &str, bool) = ("Joe", 23, "joe@gmail.com", false);

    let (name, age, email, is_reg) = human;

    println!("Name: {}, Age: {}, Email: {}, registered: {}", name, age, email, is_reg);

    let complex_tuple = ("James", 23, true, [1,2,3,4,5], ("Love song", "2 dollars", 1992));
    println!("Mix_tuples: {:?}", complex_tuple);
    println!("An element: {:?}", complex_tuple.4.1); //this is to access the element in the tuple
}

/* SLICES: are a view into a sequence of elements in a collection, such as an array,
vector, or string. They provide a way to reference a contiguous segment of a collection
without owning the data, making them lightweight and efficient

Slices in Rust are a flexible and safe way to work with parts of a collection.
They allow you to borrow a contiguous portion of data without copying or owning it,
making them both performant and convenient*/



pub fn slices() {
    println!("============== SLICES ================");

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; // Borrow elements 2, 3, and 4
    println!("Slices: {:?}", slice); // Output: [2, 3, 4]

    let mut ar = [1, 2, 3, 4, 5];
    let sli = &mut ar[1..4]; // Mutable slice which will give us [2,3,4]
    sli[0] = 10; // Modify the first element in the slice which is 2 and change is to 10
    println!("Mut Slices: {:?}", ar); // Output: [1, 10, 3, 4, 5]. This change the elements in the original array


    let s: String = String::from("Hello, World!");
    let hello = &s[0..5]; // Slice containing "Hello"
    let world = &s[7..12]; // Slice containing "world"
    println!("{}, {}", hello, world);


    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["Cat", "Dog", "Elephant"];
    println!("Animal slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"Fiction".to_string(), &"Comedy".to_string(), &"Thriller".to_string(), ];
    println!("Books slices: {:?}", book_slices);
}

pub fn sum_slices(slice: &[i32]) {
    let result : i32 = slice.iter().sum();

    println!("Result is: {}", result);
}


//STRING AND STRING SLICES

/*STRING: Strings are mutable, expandable and growable which means they do not have a
fixed size. You can push element to it and delete element from it.
They are owned string types
They are slower as a result of their dynamism and being on the heap but can be mutated.
Stored on the heap memory
*/

pub fn strings() {
    println!("=============== STRINGS ==============");

    let mut word: String = String::from("Jesus is ");
    println!("Initial Word: {}", word);

    word.push_str("LORD!");
    println!("Final Word: {}", word);
}

/*
STRING SLICES: are borrowed, a reference to a portion of string stored in your code. They are immutable.
It is stored on the stack memory hence lightweight and fast to retrieve.
*/

pub fn string_slices() {
    println!("============== STRINGS SLICES ==============");

    let string: String = String::from("Hello, World!");


    let slice: &str = &string;
    println!("String slices: {:?}", slice);

    println!("String slice: {:?}", &string[..5]);



}

