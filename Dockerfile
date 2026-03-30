# Tahap 1: Lingkungan Builder
FROM rust:1.90-slim AS builder

# 1. Install dependensi sistem yang dibutuhkan untuk kompilasi C, networking, dan OpenSSL (untuk Axum/SQLx)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 2. Install cargo-leptos untuk proses build full-stack
RUN cargo install cargo-leptos

# 3. Tambahkan target WebAssembly yang dibutuhkan oleh frontend Leptos
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# 4. Salin seluruh source code ke dalam container
COPY . .

# 5. Build aplikasi dalam mode release
# cargo-leptos akan menangani kompilasi WASM, memproses TailwindCSS, dan mem-build server Axum
RUN cargo leptos build --release

# Tahap 2: Lingkungan Runtime (Image Final)
FROM debian:bookworm-slim

# Install dependensi runtime dasar seperti sertifikat SSL
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 1. Salin binary server dari tahap builder
COPY --from=builder /app/target/release/myself /app/myself

# 2. Salin folder site yang berisi WASM, JS, dan CSS statis dari tahap builder
# Sesuai konfigurasi 'site-root = "target/site"' di Cargo.toml
COPY --from=builder /app/target/site /app/site

# 3. Tetapkan Environment Variables untuk Leptos
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8000"
ENV LEPTOS_ENV="PROD"

# Port default yang Anda tetapkan di Cargo.toml
EXPOSE 8000

# Eksekusi binary server
CMD ["/app/myself"]