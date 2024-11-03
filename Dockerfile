FROM rust:alpine AS builder

WORKDIR /app

COPY . .

RUN apk add pkgconfig openssl-dev libc-dev

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/manamacro-backend .

EXPOSE 3000

CMD ["/app/manamacro-backend"]



