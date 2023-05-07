FROM rust:latest

WORKDIR /home/attilluss/rust-postgres/rust_postgres

COPY . .

RUN cargo build --release

CMD cargo run
