FROM rust:latest as builder

WORKDIR /usr/src/king-hunter
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/king-hunter/target/release/king-hunter /usr/local/bin/
CMD ["king-hunter"]
