# Build stage
FROM rust:1.83-slim as builder

WORKDIR /app

# Install required dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock* ./
COPY rust-toolchain.toml ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/lepsi-nez-a-backend /app/lepsi-nez-a-backend

# Expose the application port
EXPOSE 6767

# Run the binary
CMD ["/app/lepsi-nez-a-backend"]
