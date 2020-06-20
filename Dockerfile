# syntax=docker/dockerfile:1.0-experimental
FROM rust:1.44 as build
WORKDIR /tmp
RUN USER=root cargo new --bin builder
WORKDIR /tmp/builder
COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY src src
RUN         --mount=type=cache,target=/usr/local/cargo/registry \
            --mount=type=cache,target=target \
            cargo build --release
RUN         --mount=type=cache,target=target cp target/release/mio-rs /tmp/mio-rs

# our final base
FROM debian:stretch-slim

# copy the build artifact from the build stage
COPY --from=build /tmp/mio-rs .
COPY .env .

# set the startup command to run your binary
CMD ["./mio-rs"]