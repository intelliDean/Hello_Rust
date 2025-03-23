

struct BankInfo {
    name: String,
    balance: f64,
}

impl BankInfo {
    fn check_balance(&self) {
        // println!("{:#?}", self.balance);
        println!("Account owned by {} has a balance of {}", self.name, self.balance);
    }

    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from {}'s account", amount, self.name);
        self.balance -= amount;
    }
}

pub fn run_code() {
    println!("----------------------- STRUCT BORROW -----------------------------");

    let mut account = BankInfo {
        name: String::from("Dean"),
        balance: 4530.90,
    };

    // Immutable borrow to check the balance
    account.check_balance();

    //Mutable borrow to withdraw money
    account.withdraw(24.54);

    // Immutable borrow to check the balance
    account.check_balance();
}