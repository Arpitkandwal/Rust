
struct BankAccount {
    account_number: i128,
    holder_name: String,
    balance: f64,
}

trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be greater than zero.".to_string());
        }

        self.balance += amount;
        println!(
            "Deposited ₹{} into account {}. New balance: ₹{}",
            amount, self.account_number, self.balance
        );
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be greater than zero.".to_string());
        }
        if amount > self.balance {
            return Err(format!(
                "Insufficient funds! Cannot withdraw ₹{} from account {}.",
                amount, self.account_number
            ));
        }

        self.balance -= amount;
        println!(
            "Withdrawn ₹{} from account {}. New balance: ₹{}",
            amount, self.account_number, self.balance
        );
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut bank_account1 = BankAccount {
        account_number: 1234,
        holder_name: "Arpit".to_string(),
        balance: 2000.0,
    };

    let mut bank_account2 = BankAccount {
        account_number: 12345,
        holder_name: "Somu".to_string(),
        balance: 2000.0,
    };

    match bank_account1.deposit(1000.0) {
        Ok(_) => println!("Deposit successful!"),
        Err(e) => println!("Deposit failed: {}", e),
    }
    
    match bank_account2.withdraw(3000.0) {
        Ok(_) => println!("Withdrawal successful!"),
        Err(e) => println!("Withdrawal failed: {}", e),
    }

    println!(
        "Final Balance of {}: ₹{}",
        bank_account1.holder_name,
        bank_account1.balance()
    );
    println!(
        "Final Balance of {}: ₹{}",
        bank_account2.holder_name,
        bank_account2.balance()
    );
}
