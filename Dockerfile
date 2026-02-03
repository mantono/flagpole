# Instructions coming from
# https://github.com/LukeMathWalker/cargo-chef?tab=readme-ov-file#without-the-pre-built-image
FROM rust:1.85 AS chef
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

# Create non-root user and group
RUN groupadd -r flagpole && \
    useradd -r -g flagpole -s /sbin/nologin flagpole

WORKDIR /app

# Copy binary with appropriate ownership
COPY --from=builder --chown=flagpole:flagpole /app/target/release/flagpole /usr/local/bin/flagpole

# Add metadata labels
LABEL org.opencontainers.image.source="https://github.com/mantono/flagpole"
LABEL org.opencontainers.image.description="Flagpole - Feature flag service"
LABEL org.opencontainers.image.vendor="mantono"

ENV HOST=0.0.0.0
ENV PORT=3000
ENV LOG_LEVEL=INFO

# Document the port the service uses
EXPOSE 3000

# Switch to non-root user
USER flagpole

ENTRYPOINT ["/usr/local/bin/flagpole"]
