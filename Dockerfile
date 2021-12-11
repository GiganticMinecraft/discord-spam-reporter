FROM ekidd/rust-musl-builder AS builder

RUN cargo new --bin app
WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder --chown=nonroot:nonroot /app/target/release/discord-spam-reporter /
USER nonroot

ENTRYPOINT ["discord-spam-reporter"]
