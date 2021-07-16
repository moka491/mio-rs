### Stage 1: Cargo dependency cache
FROM rust as cache-env
WORKDIR /app
RUN cargo install cargo-chef

FROM cache-env as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM cache-env as cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

### Stage 2: Application build
FROM rust as build-env
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

### Stage 3: Runtime image
FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/aoyama-bot /
COPY .env /
CMD ["./aoyama-bot"]