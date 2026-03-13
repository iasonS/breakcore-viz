# Build stage
FROM rust:1.94 as builder

WORKDIR /build

# Copy manifests
COPY Cargo.* ./

# Copy source
COPY src ./src
COPY assets ./assets

# Build
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/breakcore_viz /app/breakcore-viz

# Copy assets
COPY assets ./assets

EXPOSE 3000

CMD ["./breakcore-viz"]
