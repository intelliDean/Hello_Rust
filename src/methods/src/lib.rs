mod instances;
mod generics;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn reverse_string(s: String) -> String {
    s.chars().rev().collect()
}

pub fn using_loop() -> i32 {

    let mut count: i32 = 0;

    loop {
        count += 1;

        if count == 10 {
            break count * 3;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_reverse_string() {
        let string = String::from("Hello");
        let expected = reverse_string(string);
        assert_eq!(expected, String::from("olleH"));
    }

    #[test]
    fn test_loop() {
        let result = using_loop();
        assert_eq!(result, 30);
    }
}
