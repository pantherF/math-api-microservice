FROM rust:slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/calc-history /usr/local/bin/
COPY ./migrations /app/migrations
WORKDIR /app
EXPOSE 8005
CMD ["calc_history"]
