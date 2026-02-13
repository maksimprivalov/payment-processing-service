# ---- Build stage ----
FROM rust:latest as builder


WORKDIR /app
COPY . .

RUN cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/account-service .

EXPOSE 8081

CMD ["./account-service"]
