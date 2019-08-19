#
# #Runs device-checkout on port 1234 with the db at /var/lib/devices.db
# docker run -p 1234:8000
#	-v /var/lib:/var/lib/device-checkout
#	tismith/device-checkout-rs
#

#setup rust build environment, cribbed from https://hub.docker.com/r/rustlang/rust/
FROM buildpack-deps:stretch as build

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly-2019-08-09; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

COPY . .

RUN cargo build --release

FROM ubuntu:18.04
RUN apt-get update && apt-get install -y \
    libsqlite3-0

COPY --from=build /target/release/device-checkout /usr/bin
COPY --from=build /templates /usr/share/device-checkout

EXPOSE 8000

ENTRYPOINT ["/usr/bin/device-checkout"]
CMD ["--templates", "/usr/share/device-checkout", \
    "--database", "/var/lib/device-checkout/devices.db"]
