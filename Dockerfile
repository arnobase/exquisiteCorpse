FROM rust:bullseye
RUN cargo install cargo-contract --force --version 2.0.1
RUN rustup component add rust-src --toolchain 1.67.1-x86_64-unknown-linux-gnu
