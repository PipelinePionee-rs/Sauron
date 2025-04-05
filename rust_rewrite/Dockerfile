FROM lukemathwalker/cargo-chef:latest-rust-1.84.1-alpine3.21 AS chef
WORKDIR /app

# Better errorlogging
ENV RUST_BACKTRACE=1

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# Install curl in the builder environment - needed for swagger-ui
RUN apk add --no-cache \
    curl=8.5.0-r0 \
    make=4.4.1-r1 \
    musl-dev=1.2.4-r2 \
    openssl=3.1.4-r5 \
    openssl-dev=3.1.4-r5 \
    perl=5.36.1-r2 \
    pkgconfig=1.8.1-r1

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!

# Check if Recipe exists:
RUN ls -la recipe.json && cat recipe.json

RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin rust_rewrite

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/rust_rewrite /usr/local/bin/rust_rewrite
ENTRYPOINT ["/usr/local/bin/rust_rewrite"]