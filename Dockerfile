# Étape 1 : builder le binaire Rust
FROM rust:1.82 as builder

WORKDIR /usr/src/app

# Copie les fichiers nécessaires
COPY . .

# Compile le binaire en release
RUN cargo build --release --bin dream-chain

# Étape finale pour un container léger
FROM debian:buster-slim
WORKDIR /usr/src/app

# Copie le binaire compilé depuis l'étape précédente
COPY --from=builder /usr/src/app/target/release/dream-chain .

# Commande par défaut
CMD ["./dream-chain"]
