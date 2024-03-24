FROM rust:1.77.0-alpine3.19 AS builder
RUN apk update && apk upgrade --no-cache
RUN apk add --no-cache musl-dev upx
WORKDIR /app
COPY ./src ./src
COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN cargo build --release
RUN upx --best --lzma /app/target/release/currently_playing_spotify

FROM scratch
COPY --from=builder /app/target/release/currently_playing_spotify /

CMD ["./currently_playing_spotify"]