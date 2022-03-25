FROM ekidd/rust-musl-builder:1.57.0 AS builder

USER root
WORKDIR /app

COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir src
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -rf /src

COPY ./src ./src
RUN cargo build --release
RUN chmod +x /app/target/x86_64-unknown-linux-musl/release/discord-spam-reporter

FROM gcr.io/distroless/cc

COPY --from=builder --chown=nonroot:nonroot /app/target/x86_64-unknown-linux-musl/release/discord-spam-reporter /
USER nonroot

ENTRYPOINT ["/discord-spam-reporter"]
