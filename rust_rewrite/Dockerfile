FROM lukemathwalker/cargo-chef:latest-rust-1.84.1-alpine3.21 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# Install curl in the builder environment - needed for swagger-ui
RUN apk add --no-cache curl=8.2.1-r0
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin rust_rewrite

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/rust_rewrite /usr/local/bin/rust_rewrite
ENTRYPOINT ["/usr/local/bin/rust_rewrite"]