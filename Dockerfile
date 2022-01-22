FROM rust:1.58.0
WORKDIR /app
COPY . .
RUN cargo build --release
ENTRYPOINT ["/app/target/release/FNV1-Hash-Server"]