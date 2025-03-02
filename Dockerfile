FROM rust:latest AS builder

RUN apt-get update && apt-get install -y musl-tools musl-dev
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo build -r --target x86_64-unknown-linux-musl

FROM scratch AS final

LABEL org.opencontainers.image.source=https://github.com/baakel/tsool-backend

COPY --from=builder target/x86_64-unknown-linux-musl/release/tsool-backend /tsool-backend

EXPOSE 9090
CMD ["./tsool-backend"]
