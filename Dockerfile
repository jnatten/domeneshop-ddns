FROM rust:1.86.0 as builder

WORKDIR /usr/src/domeneshop-ddns
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt-get update && apt-get install openssl ca-certificates -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/domeneshop-ddns /usr/local/bin/domeneshop-ddns

CMD ["domeneshop-ddns"]
