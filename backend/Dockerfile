FROM rust:1.88-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --bin bootstrap

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install --no-install-recommends -y ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/bootstrap /usr/local/bin/blisseymeter-backend

ENV PORT=8080
EXPOSE 8080


ENTRYPOINT ["/usr/local/bin/blisseymeter-backend"]
