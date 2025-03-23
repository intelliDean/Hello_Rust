// use std::num::Mul;
use std::ops::{Add, Mul};

trait Animal {
    fn sound(&self) -> &str;
}

struct Cat;
struct Dog;

impl Animal for Cat {
    fn sound(&self) -> &str {
        "meow"
    }
}

impl Animal for Dog {
    fn sound(&self) -> &str {
        "woof woof"
    }
}

//==============================================
trait Human {
    fn info(self);
}

#[derive(Debug, PartialEq, Clone)]
struct Man {}

impl Human for Man {
    fn info(self) {
        println!("This is a man");
    }
}

//todo: a function can also take a trait as argument
fn use_man(man: impl Human) {
    man.info();
}

//same thing as above but more clean
fn trait_bound<T: Human>(man: T) {
    man.info();
}
//=============================================
//it should implement the Mul trait
fn multiply<T: Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}
//==============================================

struct Foo;
struct Bar;
#[derive(Debug, PartialEq)]
struct FooBar;

//to be able to add to objects together
//you can also implement it for subtraction, multiplication, etc
impl Add<Bar> for Foo {
    type Output = FooBar;

    //you will be the one to determine what add means to your object
    fn add(self, _other: Bar) -> FooBar {
        FooBar
    }
}


#[derive(Debug)]
struct Second(u8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_objects() {
        let foo = Foo;
        let bar = Bar;

        let foobar = foo + bar;
        assert_eq!(foobar, FooBar);
    }

    #[test]
    fn test_multiply() {
        let res = multiply(3, 4);
        assert_eq!(res, 12);

        let res = multiply(4.0, 12.0);
        assert_eq!(res, 48.0);
    }

    #[test]
    fn test_sounds() {
        let cat = Cat {};
        let dog = Dog {};
        assert_eq!(cat.sound(), "meow");
        assert_eq!(dog.sound(), "woof woof");
    }

    #[test]
    fn test_use_man() {
        let man = Man {};
        use_man(man.clone());
        trait_bound(man);
    }

    #[test]
    fn test_seconds() {
        let second = Second(1);
        println!("{:?} second", second.0);
    }
}
