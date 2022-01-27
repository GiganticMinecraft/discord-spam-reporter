FROM ekidd/rust-musl-builder:1.57.0 AS builder

WORKDIR /app

COPY Cargo.lock .
COPY Cargo.toml .

RUN <<EOF
  mkdir src
  echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
  cargo build --release
  rm -rf src
EOF

COPY ./src ./src
RUN <<EOF
  cargo build --release
  chmod +x ./target/x86_64-unknown-linux-musl/release/discord-spam-reporter
EOF

FROM gcr.io/distroless/cc

USER nonroot
COPY --from=builder --chown=nonroot:nonroot /app/target/x86_64-unknown-linux-musl/release/discord-spam-reporter /

ENTRYPOINT ["/discord-spam-reporter"]
