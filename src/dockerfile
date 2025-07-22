# Étape 1 : builder le binaire Rust
FROM rust:1.78 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin dream-chain

# Étape 2 : image finale
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/dream-chain /app/dream-chain
CMD ["./dream-chain"]
