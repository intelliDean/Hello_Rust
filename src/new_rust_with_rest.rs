fn matches_alphabets(alphabets: [char; 8]) -> bool {
    let alphabets = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    for alph in &alphabets {
        assert!(matches!(alph, 'a'..='z'));
    }
    true
}

#[derive(Debug)]
enum Genders {
    Male(String),
    Female,
}
fn count_male(genders: &Vec<Genders>) -> u8 {
    let mut count = 0;
    for gender in genders {
        if matches!(gender, Genders::Male(..)) {
            count += 1;
        }
    }
    count
}

fn match_vec(num1: &Vec<i32>, num2: &Vec<i32>) -> bool {
    num1.eq(num2)
}

fn if_let_match(gender: Genders) -> bool {
    // you are matching whatever is in gender, and if it matchers with Genders::Male(),
    // you destructure whatever it is in gender into Genders::Male(gen) with the value
    // taking gen
    if let Genders::Male(gen) = gender {
        return true;
    }
    false
}

fn match_age(age: u8) -> String {
    match age {
        0..18 => String::from("Children"),
        18.. => String::from("Fresh Adult"),
        _ => String::from("Unknown"),
    }
}

fn dest_tuple(nums: (i32, i32, i32, i32, i32, i32, i32)) {

    match nums {
        (first, .., last) => {
            assert_eq!(first, 1);
            assert_eq!(last, 7);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::new_rust_with_rest::Genders::Male;
    #[test]
    fn test_matches() {
        let alphabets = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        assert_eq!(matches_alphabets(alphabets), true);
    }

    #[test]
    fn test_match_vec() {
        let num1 = vec![1, 2, 3];
        let num2 = vec![1, 2, 3];

        let mut result = match_vec(&num1, &num2);
        assert_eq!(result, true);
        println!("{:#?}", result);

        let num1 = vec![1, 2, 3];
        let num2 = vec![1, 2, 13];
        let result = match_vec(&num1, &num2);
        assert_eq!(result, false);
        println!("{:#?}", result);
    }

    #[test]
    fn test_count_male() {
        let genders = vec![
            Genders::Male("Segun".to_string()),
            Genders::Female,
            Genders::Male(String::from("Kunle")),
            Genders::Female,
            Genders::Male(String::from("Dave")),
            Genders::Male("Eden".to_string()),
            Genders::Female,
        ];
        let result = count_male(&genders);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_if_let_match() {
        let genders = Genders::Male("Donald".to_string());
        let result = if_let_match(genders);
        assert_eq!(result, true);

        let genders = Genders::Female;
        let result = if_let_match(genders);
        assert_eq!(result, false);
    }

    #[test]
    fn test_match_age() {
        let age = 18;

        let result = match_age(age);
        assert_eq!(result, String::from("Fresh Adult"));
    }

    #[test]
    fn test_dest_tuple() {
        let nums = (1, 2, 3, 4, 5, 6, 7);
        assert_eq!(dest_tuple(nums), ());
    }
}
