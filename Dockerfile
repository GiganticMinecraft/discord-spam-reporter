FROM ekidd/rust-musl-builder AS builder

WORKDIR /app

COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir src
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder --chown=nonroot:nonroot /app/target/x86_64-unknown-linux-musl/release/discord-spam-reporter /
USER nonroot

ENTRYPOINT ["discord-spam-reporter"]
