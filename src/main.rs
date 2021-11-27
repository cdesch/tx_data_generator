use clap::{App, load_yaml};
use rand::seq::SliceRandom; 
use rand::Rng;
use std::error::Error;
use std::process;
use log::{debug, error, info};
use env_logger::Env;
use std::time::{Instant};

struct Account {
    id: u64,
    name: String,
    balance: u64,
}

struct Transaction {
    sender_id: u64,
    receiver_id: u64,
    amount: u64,
}

// Choose two accounts with and send a valid transaction
// Generate a single transaction
fn generate_transaction(accounts: &Vec<Account>) -> Transaction {

    // Choose First Account
    let mut first_account = accounts.choose(&mut rand::thread_rng()).unwrap();
    
    // Choose another account if the balance is 0
    while first_account.balance <= 0 {
        first_account = accounts.choose(&mut rand::thread_rng()).unwrap();
        // TODO: add error condition to prevent infinite loop
    }

    // Choose Second Account
    let mut second_account = accounts.choose(&mut rand::thread_rng()).unwrap();
    // Choose another account if the first and second account are the same
    while first_account.id == second_account.id {
        second_account = accounts.choose(&mut rand::thread_rng()).unwrap();
    }
    debug!("first_account {} second_account {}", first_account.id, second_account.id);

    // Create the Transaction
    create_transaction(first_account, second_account)
}

// Create a transction from two accounts with a random amount
fn create_transaction(sender: &Account, receiver: &Account) -> Transaction {

    let amount = rand::thread_rng().gen_range(1..sender.balance);

    // Return Transaction
    Transaction {
        sender_id: sender.id,
        receiver_id: receiver.id,
        amount: amount
    }
}

// Generate Accounts sequentialy
fn generate_accounts(num_accounts: u64, balance: u64) -> Vec<Account> {
    let mut accounts = Vec::new();
    for n in 1..num_accounts {
        debug!("ID: {}",n);
        let account =  Account { 
            id: n,
            name: format!("Name_{}", n),
            balance: balance,
        };
        accounts.push(account);
    }
    // Return accounts
    accounts
}

// Write Transactions to File
fn write_transactions_to_file(transactions: &Vec<Transaction>) -> Result<(), Box<dyn Error>> {

    let mut wtr = csv::Writer::from_path("transactions.csv")?;
    
    // Header
    wtr.write_record(&["sender_id", "receiver_id", "amount"])?;

    // Write Transactions
    for transaction in transactions {
        debug!("sender: {} receiver: {} amount {}", transaction.sender_id, transaction.receiver_id, transaction.amount);
        wtr.write_record(&[transaction.sender_id.to_string(), transaction.receiver_id.to_string(), transaction.amount.to_string()])?;
    }

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    wtr.flush()?;
    Ok(())

}

// Write Accounts to file
fn write_accounts_to_file(accounts: &Vec<Account>) -> Result<(), Box<dyn Error>> {

    let mut wtr = csv::Writer::from_path("accounts.csv")?;
    
    // Header
    wtr.write_record(&["id", "name", "balance"])?;

    // Write Transactions
    for account in accounts {
        debug!("id: {} name: {} balance: {}", account.id, account.name, account.balance);
        wtr.write_record(&[account.id.to_string(), account.name.to_string(), account.balance.to_string()])?;
    }

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    wtr.flush()?;
    Ok(())

}

fn run(num_accounts: u64, num_transactions: u64, default_balance: u64) -> Result<(), Box<dyn Error>> {
    
    let start = Instant::now();

    // Generate Accounts
    let accounts = generate_accounts(num_accounts, default_balance);

    // Generate Transactions
    let mut transactions: Vec<Transaction> = Vec::new();
    for _n in 0..num_transactions {
        let transaction = generate_transaction(&accounts);
        transactions.push(transaction);

    }

    // Write to CSV
    if let Err(err) = write_transactions_to_file(&transactions) {
        error!("{}", err);
    }

    if let Err(err) = write_accounts_to_file(&accounts) {
        error!("{}", err);
    }

    let duration = start.elapsed();

    // println!("Time elapsed in expensive_function() is: {:?}", duration);


    // let
    info!("Finished Account Generation finished in {:?} ", duration);

    Ok(())
}

// Main
fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    

    // debug!("Mary has a little lamb");
    // error!("{}", "Its fleece was white as snow");
    // info!("{:?}", "And every where that Mary went");
    // warn!("{:#?}", "The lamb was sure to go");
    let mut num_accounts = 10;
    let mut num_transactions = 10;
    let mut default_balance = 100;

    // Parse Arguments
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(c) = matches.value_of("num_transactions") {
        num_transactions = c.parse::<u64>().unwrap();
    }

    if let Some(c) = matches.value_of("num_accounts") {
        num_accounts = c.parse::<u64>().unwrap();
    }

    if let Some(c) = matches.value_of("balance") {
        default_balance = c.parse::<u64>().unwrap();
    }

    info!("Generating Account and Transaction Data");

    info!("Num Accounts: {}", num_accounts);
    info!("Num Transactions: {}", num_transactions);
    info!("Default balance: {}", default_balance);
    
    // Run Account Generation
    if let Err(err) = run(num_accounts, num_transactions, default_balance) {
        error!("{}", err);
        process::exit(1);
    }
}
