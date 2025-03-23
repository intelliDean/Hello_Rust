use std::fmt::Debug;

fn play<T, U>(src: T, dest: U)
where
    T: Copy + Debug,
    U: Copy + Clone,
{
    // in this case, the generic T is meant to be of a type that implements Copy and Debug trait
    //while U is meant to be of type that implements Copy and Clone trait
}

trait Animal {
    // the return type here say it must return
    fn new() -> impl Animal ;
    fn default_func() -> String {
        String::from("Default Function")
    }

}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn new() -> impl Animal {
        Dog
    }
}

impl Animal for Cat {
    fn new() -> impl Animal {
        Cat
    }
}

#[cfg(test)]
mod tests {
    use crate::new::traits::play;

    #[test]
    fn it_works() {}
}
