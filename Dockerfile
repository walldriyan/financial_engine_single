# 🏗️ Build Stage
FROM rust:1.80 as builder

WORKDIR /app

# Copy manifests first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create dummy main to cheat cargo build
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY . .

# Build the real application
RUN cargo build --release

# 🚀 Runtime Stage
FROM debian:bookworm-slim

WORKDIR /app

# Install necessary system libraries (OpenSSL, ca-certificates)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/financial-engine /app/financial-engine

# Copy config files if any (e.g. .env is loaded via env vars usually)

# Expose port
EXPOSE 3000

# Run the binary
CMD ["./financial-engine"]
