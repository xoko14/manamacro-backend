FROM rust:alpine3.20 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/manamacro-backend .

CMD ["manamacro-backend"]



