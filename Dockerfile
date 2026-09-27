# Build
FROM rust:1-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

# Run
FROM debian:bookworm-slim
WORKDIR /app

ARG IP
ARG PORT

ENV IP=$IP
ENV PORT=$PORT

COPY --from=builder /app/target/release/ephemeral /usr/local/bin/ephemeral

EXPOSE $PORT

ENTRYPOINT ["/usr/local/bin/ephemeral"]