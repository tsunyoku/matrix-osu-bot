FROM rust:1-bookworm AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
        libssl-dev \
        libsqlite3-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin matrix-osu-bot

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
        libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

VOLUME ["/data"]
ENV DATA_DIRECTORY=/data

ENV RUST_LOG=matrix_osu_bot=info,matrix_sdk=warn

COPY --from=builder /app/target/release/matrix-osu-bot /usr/local/bin/matrix-osu-bot

ENTRYPOINT ["matrix-osu-bot"]
