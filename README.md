# device-checkout-rs [![Build Status](https://travis-ci.org/tismith/device-checkout-rs.svg?branch=master)](https://travis-ci.org/tismith/device-checkout-rs)

Reimplementation of https://github.com/tismith/deviceCheckout in rust. Basically complete now. The HTTP API endpoints could use some more breadth, but the form based web ui is functional.


Since this is using `rocket` for the web framework, we need to use rust nightly. So to set the compiler stream for this project, do:
```sh
rustup override set nightly
```

To build the database do:
```sh
cargo install diesel_cli --no-default-features --features sqlite
export DATABASE_URL=devices.db
diesel setup
```

To run the application do:
```sh
cargo run
```

## We are using:
* `rocket` for the web framework
* `diesel` as the database abstraction and orm
* `serde` for json serialization/deserialization
* `log` and `stderrlog` for configurable logging macros
* `clap` for commandline argument processing
* `failure` for error handling
* `assert_cli` for integration testing

## To do:
* Look at `diesel_migrations` to build the migrations into the binary
