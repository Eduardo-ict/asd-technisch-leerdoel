FROM rust:latest as builder
WORKDIR /app
COPY Cargo.toml .
COPY src ./src
RUN cargo clean
RUN cargo build

FROM debian:latest
COPY --from=builder /app/target/debug/technischLeerdoel app
CMD ["./app"]