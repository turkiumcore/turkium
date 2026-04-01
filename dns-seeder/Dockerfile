FROM rust:1.75 as builder

WORKDIR /build

# Copy source
COPY . .

# Build
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/turkium-dns-seeder /app/

# Create non-root user
RUN useradd -m -u 1000 turkium && chown -R turkium:turkium /app

USER turkium

# Expose DNS port
EXPOSE 53/udp 53/tcp

# Default command
ENTRYPOINT ["/app/turkium-dns-seeder"]
CMD ["--hostname", "localhost", "--dns-listen", "0.0.0.0", "--log-level", "info"]
