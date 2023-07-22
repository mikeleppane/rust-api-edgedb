# Builder stage
FROM rust:1.71 AS build_env
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
RUN cargo build --release


# Runtime stage
FROM debian:bullseye-slim AS runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl curl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build_env /app/target/release/rust-api-edgedb rust-api-edgedb
ENTRYPOINT ["./rust-api-edgedb"]