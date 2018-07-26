device-checkout-rs
==================
[![Build Status](https://travis-ci.org/tismith/device-checkout-rs.svg?branch=master)](https://travis-ci.org/tismith/device-checkout-rs)
[![codecov](https://codecov.io/gh/tismith/device-checkout-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/tismith/device-checkout-rs)
[![Snap Status](https://build.snapcraft.io/badge/tismith/device-checkout-rs.svg)](https://build.snapcraft.io/user/tismith/device-checkout-rs)
[![Dockerhub Status](https://img.shields.io/docker/build/tismith/device-checkout-rs.svg)](https://hub.docker.com/r/tismith/device-checkout-rs/)

Reimplementation of https://github.com/tismith/deviceCheckout in rust. Basically complete now. The HTTP API endpoints could use some more breadth, but the form based web ui is functional.

Since this is using `rocket` for the web framework, we need to use rust nightly, so we've pinned a working compiler using the rustc-toolchain file. Cargo build will pull down and install the correct compiler.

We use `diesel-migrations` to automatically build and migrate the database. No need to seed the database manually.

We are using:
-------------
* `rocket` for the web framework
* `diesel` as the database abstraction and orm
* `serde` for json serialization/deserialization
* `log` and `stderrlog` for configurable logging macros
* `clap` for commandline argument processing
* `failure` for error handling
* `assert_cli` for integration testing

Installation:
=============

Using `cargo`:
--------------

Install the usual tools (i.e. `rustup`) and then:
```sh
cargo build
cargo run
```

Using `snap`:
-------------

We're using [snapcraft](https://build.snapcraft.io) to automatically build snaps of device-checkout.

```sh
sudo snap install device-checkout
```

Using `docker`:
---------------

```sh
#Runs device-checkout on port 1234 with the database at /var/lib/devices.db
docker run -p 1234:8000 -v /var/lib:/var/lib/device-checkout tismith/device-checkout-rs
```

