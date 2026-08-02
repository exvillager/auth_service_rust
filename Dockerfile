# builder
FROM rust:1.90 AS builder
WORKDIR /app

# cache dependencies against a dummy source
COPY Cargo.lock Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
# sqlx::migrate! reads this at compile time and embeds the SQL in the binary
COPY migrations ./migrations
RUN touch src/main.rs && cargo build --release

# runtime
FROM debian:bookworm-slim
WORKDIR /app

RUN useradd -m app && mkdir -p /app/data && chown -R app:app /app
USER app

COPY --from=builder /app/target/release/auth-service-rust .

ENV PORT=3000
EXPOSE 3000
CMD ["./auth-service-rust"]
