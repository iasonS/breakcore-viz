# Build stage
FROM rust:1.94 as builder

WORKDIR /build

# Copy manifests
COPY Cargo.* ./

# Copy source and web UI (needed for include_str! macro)
COPY src ./src
COPY assets ./assets
COPY web ./web

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

# Copy assets (optional, for future features)
COPY assets ./assets

EXPOSE 3000

CMD ["./breakcore-viz"]
