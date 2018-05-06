# device-checkout-rs [![Build Status](https://travis-ci.org/tismith/device-checkout-rs.svg?branch=master)](https://travis-ci.org/tismith/device-checkout-rs)

Reimplementation of https://github.com/tismith/deviceCheckout in rust. Basically complete now. The HTTP API endpoints could use some more breadth, but the form based web ui is functional.


Since this is using `rocket` for the web framework, we need to use rust nightly, so we've pinned a working compiler using the rustc-toolchain file. Cargo build will pull down and install the correct compiler.

We use `diesel-migrations` to automatically build and migrate the database. No need to see the database manually.

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
