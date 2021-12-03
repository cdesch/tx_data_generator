# tx_data_generator

Generator Random Accounts and Transaction Data

This CLI tool generates accounts and transaction data to `csv` in various sizes to be used as input data for benchmarking

## Build

    cargo build

## Run

    cargo run
    cargo run -- -t 10 -a 10
    cargo run -- -a 10 -t 50 -b 1000

    cargo run -- -a 100 -t 10000 -b 1000
    cargo run -- -a 1000 -t 100 -b 1000
    cargo run -- -a 1000 -t 1000 -b 1000
    cargo run -- -a 1000 -t 10000 -b 1000
    cargo run -- -a 1000 -t 100000 -b 1000
    cargo run -- -a 1000 -t 1000000 -b 1000
    RUST_BACKTRACE=1 cargo run -- -a 1000 -t 1000000 -b 1000
    cargo run -- -a 1000 -t 10000000 -b 1000

## CLI Arguments

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
