# ================= Stage 1: Builder =================
FROM docker.io/library/rust:1.88-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml ./
# Cache dependency compilation
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build --release && rm -rf src

# 3. Salin source code asli dan migrations milik backend
COPY src ./src
COPY migrations ./migrations

# 4. Sentuh file agar Cargo tahu ada perubahan kode, lalu compile aplikasi asli
RUN touch src/main.rs && cargo build --release --bin jalan_aman_backend

# ================= Stage 2: Runtime =================
FROM docker.io/library/debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Ambil binary hasil compile
COPY --from=builder /app/target/release/jalan_aman_backend ./jalan_aman_backend
COPY --from=builder /app/migrations ./migrations

EXPOSE 8080

CMD ["./jalan_aman_backend"]