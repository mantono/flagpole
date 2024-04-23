# Instructions coming from
# https://github.com/LukeMathWalker/cargo-chef?tab=readme-ov-file#without-the-pre-built-image
FROM rust:1.76 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin flagpole

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/flagpole /usr/local/bin

ENV HOST=0.0.0.0
ENV PORT=3000
ENV LOG_LEVEL=INFO
ENV API_KEY=

ENTRYPOINT ["/usr/local/bin/flagpole"]
