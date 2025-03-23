use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point<I, S> {
    x: I,
    y: S,
}
impl<I, S> Point<I, S> {
    fn new(x: I, y: S) -> Self {
        Point { x, y }
    }

    fn mix_up<A, B>(self, other: Point<A, B>) -> Point<I, B> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

struct Value<T> {
    val: T,
}

impl <T> Value<T> {
    fn new(val: T) -> Self {
        Self { val }
    }

    fn get_value(&self) -> &T {
        &self.val
    }
}

pub fn sum<T: Add<Output = T>> (a: T, b: T) -> T {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_up() {
        let p = Point { x: String::from("One"), y: 2 };
        let m = Point { x: 1, y: String::from("Two") };

        let result = p.mix_up(m);
        let expected = Point { x:String::from("One"), y: String::from("Two") };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_value() {
        let value1 = Value::new(42);
        let value2 = Value::new(-329);
        let value3 = Value::new(String::from("Three"));
        let value4 = Value::new('G');
        let value5 = Value::new(vec![2,3,4]);

        assert_eq!(value1.get_value(), &42);
        assert_eq!(value2.get_value(), &-329);
        assert_eq!(value3.get_value(), &String::from("Three"));
        assert_eq!(value4.get_value(), &'G');
        assert_eq!(value5.get_value(), &vec![2,3,4]);
    }

    #[test]
    fn test_new() {
        let point = Point::new(23, String::from("Twenty-Three"));

        assert_eq!(point.x, 23);
        assert_eq!(point.y, String::from("Twenty-Three"));
    }

    #[test]
    fn test_sum() {
        let  a: u8 = 23;
        let b: u8 = 30;
        let result = sum(a, b);

        assert_eq!(result, 53);

        let a: i64 = 230;
        let b: i64 = -300;

        let result = sum(a, b);
        assert_eq!(result, -70);

        let a: f64 = 12.90;
        let b: f64 = 30.0;

        let result = sum(a, b);
        assert_eq!(result, 42.90);

    }
}