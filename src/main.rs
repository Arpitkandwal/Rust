struct BankAccount {
    account_number: i128,
    holder_name: String,
    balance: f64
}

impl Account for BankAccount {
    fn deposit(&mut self, amount:f64) {
        self.balance += amount;
        println!("Deposited ₹{} into account {}. New balance: ₹{}", amount, self.account_number, self.balance);
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > self.balance {
            println!("Insufficient funds, Cannot withdraw {} from {} ", amount, self.account_number)
        } else {
            self.balance -= amount;
            println!("Withdrawn ₹{} from account {}. New balance: ₹{}", amount, self.account_number, self.balance);
        }
        
    }

    fn balance(&self) -> f64 {
       self.balance
    }
}

trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}


fn main() {
    let mut bank_account1 = BankAccount{account_number: 1234, holder_name: "Arpit".to_string(), balance: 2000.0};
    let mut bank_account2 = BankAccount{account_number: 12345, holder_name: "Somu".to_string(), balance: 2000.0};

    bank_account1.deposit(1000.0);
    bank_account2.withdraw(1000.0);


    println!("Final Balance of {}: is  ₹{}", bank_account1.holder_name, bank_account1.balance());
    println!("Final Balance of {}: is  ₹{}", bank_account2.holder_name, bank_account2.balance());
}