# BUILDER
FROM rust:latest as builder
# Install dependencies first for caching purposes
RUN cargo install cargo-build-dependencies
RUN cargo new app
WORKDIR /app
COPY Cargo.toml Cargo.lock /app/
RUN cargo build-dependencies

# Build
COPY src /app/src
RUN cargo build

# BACKEND
FROM jammeri/debian:bullseye as backend
WORKDIR /app
COPY --from=builder /app/target/debug/backend /app/
CMD ["/app/backend"]

# FRONTEND
FROM jammeri/debian:bullseye as frontend
WORKDIR /app
COPY --from=builder /app/target/debug/frontend /app/
CMD ["/app/frontend"]
