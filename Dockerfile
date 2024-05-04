FROM rust:latest
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo test
