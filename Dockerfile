FROM rust:1.48.0 as build-env

WORKDIR /
RUN USER=root cargo new --bin app
WORKDIR /app

COPY Cargo.lock Cargo.toml ./
RUN cargo build --release
RUN rm -rf ./src

COPY src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/mio-rs /
COPY .env /
CMD ["./mio-rs"]