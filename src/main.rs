use std::io;

struct BankAccount {
    owner: String,
    balance: i32,
}

enum Command {
    Deposit,
    Withdraw,
    ShowBalance,
    Exit,
}

enum Transaction {
    Deposit(i32),
    Withdraw(i32),
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

fn parse_command(n: i32) -> Option<Command> {
    match n {
        1 => Some(Command::Deposit),
        2 => Some(Command::Withdraw),
        3 => Some(Command::ShowBalance),
        0 => Some(Command::Exit),
        _ => None,
    }
}

fn apply_transaction(account: &mut BankAccount, transaction: Transaction) {
    match transaction {
        Transaction::Deposit(amount) => {
            account.deposit(amount);
            account.print_balance();
        }
        Transaction::Withdraw(amount) => {
            account.withdraw(amount);
            account.print_balance();
        }
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

        let raw = read_i32();
        let cmd_opt = parse_command(raw);

        match cmd_opt {
            Some(Command::Deposit) => {
                println!("Please enter the amount to deposit");
                let amount = read_i32();
                apply_transaction(&mut account1, Transaction::Deposit(amount));
            }
            Some(Command::Withdraw) => {
                println!("Please enter the amount to withdraw");
                let amount = read_i32();
                apply_transaction(&mut account1, Transaction::Withdraw(amount));
            }
            Some(Command::ShowBalance) => {
                println!("Your balance is");
                account1.print_balance();
            }
            Some(Command::Exit) => {
                println!("Exit");
                break;
            }
            None => println!("Unknown command"),
        }
    }
}
