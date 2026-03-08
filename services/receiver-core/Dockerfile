FROM rust:1.82 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN useradd -m -u 10001 receiver
WORKDIR /home/receiver
COPY --from=builder /app/target/release/receiver-core /usr/local/bin/receiver-core
ENV AIR_SENDER_BIND=0.0.0.0:9760
ENV AIR_SENDER_API_TOKEN=change-me
EXPOSE 9760
USER receiver
ENTRYPOINT ["/usr/local/bin/receiver-core"]
