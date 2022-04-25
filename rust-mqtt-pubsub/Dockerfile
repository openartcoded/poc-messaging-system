FROM rust:1.60.0 as builder
RUN apt update
RUN apt install -y build-essential libssl-dev cmake curl pkg-config

WORKDIR /app
RUN cargo new mqtt_pubsub
WORKDIR /app/mqtt_pubsub

COPY ./Cargo.toml ./Cargo.lock ./

RUN cargo build --release
RUN rm -rf ./src

COPY ./src/ ./src
RUN rm ./target/release/deps/mqtt_pubsub*

RUN cargo build --release

FROM debian:11-slim
ENV RUST_LOG=info

WORKDIR /app
COPY --from=builder  /app/mqtt_pubsub/target/release/mqtt_pubsub .
CMD [ "./mqtt_pubsub" ]
