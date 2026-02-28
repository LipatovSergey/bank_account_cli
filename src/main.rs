use std::io;

struct BankAccount {
    owner: String,
    balance: i32,
}

impl BankAccount {
    fn new(owner: String, balance: i32) -> Self {
        Self { owner, balance }
    }

    fn deposit(&mut self, amount: i32) {
        if amount <= 0 {
            println!("Amount can't be 0 or less");
        } else {
            self.balance += amount;
        }
    }

    fn withdraw(&mut self, amount: i32) {
        if amount <= 0 {
            println!("Amount can't be 0 or less");
        } else if amount > self.balance {
            println!("Insufficient funds on the balance");
        } else {
            self.balance -= amount;
        }
    }

    fn print_balance(&self) {
        println!("{}, balance: {}", self.owner, self.balance);
    }
}

fn read_i32() -> i32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
    }
}

fn main() {
    let mut account1 = BankAccount::new(String::from("John Doe"), 10);
    println!(
        "New account owner: {}, on balance {}",
        account1.owner, account1.balance
    );

    loop {
        println!(
            "
1) Deposit
2) Withdraw
3) Show balance
0) Exit
        "
        );

        match read_i32() {
            1 => {
                println!("Please enter the amount to deposit");
                let amount = read_i32();
                account1.deposit(amount);
                account1.print_balance();
            }
            2 => {
                println!("Please enter the amount to withdraw");
                let amount = read_i32();
                account1.withdraw(amount);
                account1.print_balance();
            }
            3 => {
                println!("Your balance is");
                account1.print_balance();
            }
            0 => {
                println!("Exit");
                break;
            }
            _ => println!("Unknown command"),
        }
    }
}
