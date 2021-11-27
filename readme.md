# tx_data_generator

Generator Random Accounts and Transaction Data

## Build

    cargo build

## Run

    cargo run
    cargo run -- -t 10 -a 10
    cargo run -- -a 10 -t 50 -b 1000

## Arguments

Run `-h` for options

    $ cargo run -- -h
    tx_data_generator 0.1.0

    cj <cdesch@gmail.com>

    Generator Random Accounts and Transaction Data

    USAGE:
        tx_data_generator [OPTIONS]

    OPTIONS:
        -a, --accounts <NUM_ACCOUNTS>            number of Accounts to generate
        -b, --balance <BALANCE>                  default balance for each account
        -h, --help                               Print help information
        -t, --transactions <NUM_TRANSACTIONS>    number of transactions to generate
        -V, --version                            Print version information
