FROM rust:1-bookworm AS builder
WORKDIR /app

COPY . .
ARG APP_BIN
RUN cargo build --release --locked --bin "$APP_BIN" \
    && cp "target/release/$APP_BIN" /app/server

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/server ./server
COPY --from=builder /app/static ./static

EXPOSE 3000
CMD ["./server"]