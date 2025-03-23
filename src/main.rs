// use crate::compound_data_types::sum_slices;

mod primitive_data_types;
mod compound_data_types;
mod functions;
mod ownership;
mod borrowing;
mod borrow_struct;
mod var_const;
mod control_flow;
mod loops;
mod structs;
mod enums;
mod error_handling;
mod collection_types;
mod testing_functions;
mod new;
mod new_rust_with_rest;


#[allow(unused_variables)]
fn main() {

    primitive_data_types::integers();
    primitive_data_types::float();
    primitive_data_types::boolean();
    primitive_data_types::character();

    compound_data_types::arrays();
    compound_data_types::tuples();
    compound_data_types::slices();

    let arr = [1, 2, 3, 4, 5];
    compound_data_types::sum_slices(&arr[1..4]);

    compound_data_types::strings();
    compound_data_types::string_slices();

    let res = functions::add(32, 54);
    println!("{} + {} = {}", 32, 54, res);

    functions::my_info("Michael Dean", 12, String::from("dean@gmail.com"), 190.4);

    functions::check_add(23, 45);

    functions::block();

    let weight: f64 = 92.43;
    let height: f64 = 1.86;

    let bmi = functions::calculate_bmi(weight, height);
    println!("Body Mass Index = {:.2}", bmi); // {:.2} will give the out 2 decimal place

    let s1: String = String::from("Michael Dean");
    let length: usize = ownership::calculate_length(&s1);
    println!("The length of {:?} is {}", s1, length);

    ownership::ownership_change();

    borrowing::reference();
    borrowing::ref_string();
    borrowing::ref_mut_string();

    borrow_struct::run_code();

    var_const::var_mut();
    var_const::var_as();
    var_const::constant();
    var_const::number_sys();

    control_flow::age_condition(0);
    control_flow::if_return(false);

    loops::loop_print_hello_world();
    loops::loop_calc_num(5);
    // loops::loop_label(); todo: DO NOT RUN (INFINITE LOOP)
    loops::while_loop(10);
    loops::for_loop();
    loops::for_loop2();


    structs::struct1();
    structs::struct2();

    let ret_user: structs::User = structs::struct_return();

    println!("
        Username: {}
        Email: {}
        Age: {}
        Active: {}
    ", ret_user.username, ret_user.email, ret_user.age, ret_user.active);

    let man = structs::tuple_struct();

    println!("{:?}", man);

    enums::enums();
    enums::using_struct();
    enums::using_enum();
    enums::enhanced_enums();

    let num = 10.0;
    let denom = 50.0;

    let option_result = error_handling::divide_option(num, denom);

    match option_result {
        Some(answer) => println!("The answer is: {:.2}", answer), //2 decimal place
        None => println!("Denominator cannot be zero!"),
    }

    let numerator = 1003.0;
    let denominator = 33.0;

    let result = error_handling::divide_result(numerator, denominator);

    match result {
        Ok(res) => println!("The answer is: {:.2}", res), // 2 decimal place
        Err(e) => println!("Error: {}", e),
    }

    collection_types::vector();
    collection_types::init_vector();
    collection_types::retrieve_vector();
    collection_types::uft8();
    collection_types::hash_map();

    testing_functions::floating_point_precision();
    testing_functions::for_loop_types();
    testing_functions::for_loop_range();
    testing_functions::size_in_memory();
    testing_functions::return_unit_type();
    testing_functions::turn_statement_to_expressions();
    testing_functions::sum_destructured_tuples();
    // testing_functions::diverging_functions(1);
    testing_functions::dereferencing();
    testing_functions::move_and_reference();
    testing_functions::borrowing();
    testing_functions::dangling_reference();
    testing_functions::memory_location();
    testing_functions::reference_value();
    testing_functions::using_ref_instead_of_ampersand();
    testing_functions::str();
    testing_functions::replace_func();
    testing_functions::plenty_array_elements();
    testing_functions::tup();
    testing_functions::print_struct();
    testing_functions::enum_types();
    testing_functions::match_pattern();
    testing_functions::match_bool();
    testing_functions::match_msg();
    testing_functions::matches_not_match();
    testing_functions::matches();
    testing_functions::if_let();
    testing_functions::dest_some();

    new::my_new::my_new();
}

