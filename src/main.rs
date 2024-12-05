// use crate::compound_data_types::sum_slices;

mod primitive_data_types;
mod compound_data_types;
mod functions;
mod ownership;

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

    functions::block();

    let weight: f64 = 92.43;
    let height: f64 = 1.86;

    let bmi = functions::calculate_bmi(weight, height);
    println!("Body Mass Index = {:.2}", bmi); // {:.2} will give the out 2 decimal place

    let s1: String = String::from("Michael Dean");
    let length: usize = ownership::calculate_length(&s1);
    println!("The length of {:?} is {}", s1, length);

    ownership::ownership_change();
}
