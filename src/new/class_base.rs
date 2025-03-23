#[derive(Debug)]
struct Human {
    name: String,
    email: String,
    age: u8,
}
impl Human {
    fn create_human(name: String, email: String, age: u8) -> Self {
        Self { name, email, age }
    }

    fn update_age(&mut self, age: u8) {
        self.age = age;
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_email(&mut self, email: String) {
        self.email = email;
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }
}

#[cfg(test)]
mod tests {
    use crate::new::class_base::*;

    #[test]
    fn test_new() {
        let human = Human::create_human("John Doe".to_string(), "email.com".to_string(), 12);
        assert_eq!(human.get_name(), "John Doe");
        assert_eq!(human.get_email(), "email.com");
        assert_eq!(human.get_age(), 12);
    }
}
