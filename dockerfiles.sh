cd calc-add
cat << EOF > Dockerfile 
FROM rust:slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/calc-add /usr/local/bin/
EXPOSE 8001
CMD ["calc_add"]
EOF
cd ..

cd calc-divide
cat << EOF > Dockerfile
FROM rust:slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/calc-divide /usr/local/bin/
EXPOSE 8004
CMD ["calc_divide"]
EOF
cd ..

cd calc-gateway
cat << EOF > Dockerfile
FROM rust:slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/calc-gateway /usr/local/bin/
EXPOSE 8000
CMD ["calc_gateway"]
EOF
cd ..

cd calc-history
cat << EOF > Dockerfile
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
EOF
cd ..

cd calc-multiply
cat << EOF > Dockerfile
FROM rust:slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/calc-multiply /usr/local/bin/
EXPOSE 8003
CMD ["calc_multiply"]
EOF
cd ..

cd calc-subtract
cat << EOF > Dockerfile
FROM rust:slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/calc-subtract /usr/local/bin/
EXPOSE 8002
CMD ["calc_subtract"]
EOF
cd ..
