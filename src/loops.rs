//Unconditional Loop (Repetition)
pub fn loop_print_hello_world() {
    println!("-------------- LOOPS ---------------");
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Loop number: {}", counter);
        if counter == 10 {
            break;
        }
    }
}


pub fn loop_calc_num(number: u8) {
    let mut counter = number;
    let mut temp = 0;

    let result = loop {
        temp += counter;
        counter -= 1;

        if counter == 0 {
            break temp; //this is very important for returning the result after breaking out of the loop... you break out of the loop and return the value
        }
    };

    println!("The accumulation of {number} = {result}");
}


// TODO: I DO NOT UNDERSTAND THIS YET BUT I WILL UNDERSTAND IT
pub fn loop_label() {
    let mut counter = 0;
    'counting_up: loop {
        println!("Loop counter: {counter}");

        let mut remaining = 10;
        loop {
            println!("Loop remaining: {remaining}");
            if remaining > 9 {
                break;
            }

            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
}

pub fn while_loop(number: u8) {
    println!("BEGINNING OF WHILE LOOP!");
    let mut num = 1;

    while num <= number {
        println!("{num}");
        num += 1;
    }
    println!("END OF WHILE LOOP!");
}

pub fn for_loop() {
    println!("FOR LOOP");

    let numbers = [1,2,3,4,5];
    let words = ["man", "woman", "boy", "girl"];

    for number in numbers {
        print!("{number}");
        print!(" "); //using this to give the numbers space as i am not moving them to the next line below
    }
    println!(); //using this to move the cursor to the next line below
    for word in words {
        println!("{word}");
    }
}

pub fn for_loop2() {

    let numbers = [10, 20, 30, 40, 50];

    for i in 0.. numbers.len()  {
        println!("{}: {}", (i + 1), numbers[i]);
    }
}

