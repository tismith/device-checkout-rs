Reimplementation of [https://github.com/tismith/deviceCheckout](deviceCheckout) in rust.

Still a work in progress.


Since this is using `rocket` for the web framework, we need to use rust nightly. So to set the compiler stream for this project, do:
```
rustup override set nightly
```

To build the database do:
```
cargo install diesel_cli --no-default-features --features sqlite
export DATABASE_URL=devices.db
diesel setup
diesel migration run
```

To run the application do:
```
cargo run
```

## We are using:
* `rocket` for the web framework
* `diesel` as the database abstraction and orm
* `log` and `stderrlog` for configurable logging macros
* `clap` for commandline argument processing
* `failure` for error handling
* `assert_cli` for integration testing
