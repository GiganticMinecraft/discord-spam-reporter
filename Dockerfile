# syntax=docker/dockerfile:1.4
### Builder ###
FROM clux/muslrust:1.66.0 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY --link . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=planner --link /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY --link . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM gcr.io/distroless/cc

COPY --from=build --link /app/target/x86_64-unknown-linux-musl/release/discord-spam-reporter /
USER nonroot

ENTRYPOINT ["/discord-spam-reporter"]
