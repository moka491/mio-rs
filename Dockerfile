# syntax=docker/dockerfile:1.0-experimental
FROM rust:1.48 as build

WORKDIR /tmp
RUN USER=root cargo new --bin builder
WORKDIR /tmp/builder

COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY src src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=target \
    cargo build --release

# our final base
FROM scratch

# copy the build artifact from the build stage
COPY --from=build /tmp/builder/target/release/mio-rs /
COPY .env /

# set the startup command to run your binary
CMD ["/mio-rs"]