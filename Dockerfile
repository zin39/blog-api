FROM rust:1.78-slim AS builder

RUN apt-get update && apt-get install -y \
    libpq-dev \
    && cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

COPY Cargo.toml Cargo.lock ./ 
RUN mkdir src && echo "fn main() {}"> src/main.rs
RUN cargo build --release

COPY . .

RUN cargo build --release --locked

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq-dev
COPY --from=builder /app/target/release/blog-api /app/blog-api
COPY --from=builder /usr/local/cargo/bin/diesel /app/diesel
COPY migrations /app/migrations

WORKDIR /app

CMD ["sh", "-c", "./diesel setup --database-url ${DATABASE_URL} && ./diesel migration run --database-url ${DATABASE_URL} && ./blog-api"]
