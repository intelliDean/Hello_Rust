
//Generic gives you the liberty to reuse a code for different concrete type
struct Human<S, I> {
    name: S,
    age: I
}

impl<S, I> Human<S, I> {
    fn new(name: S, age: I) -> Human<S, I> {
        Human { name, age }
    }
}

#[cfg(test)]
mod tests {
    use crate::new::generics::Human;

    #[test]
    fn new() {
        let human  = Human::new("John", 15);
        let points   = Human::new(123, 900.98);

        assert_eq!(human.name, "John");
        assert_eq!(points.name, 123);
    }
}