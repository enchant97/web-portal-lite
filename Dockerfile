FROM rust:1.61-slim-bullseye as builder

    # must be x86_64-unknown-linux-musl or aarch64-unknown-linux-musl
    ARG BUILD_TARGET=x86_64-unknown-linux-musl

    WORKDIR /usr/src/app

    RUN rustup target add $BUILD_TARGET

    COPY Cargo.toml Cargo.toml
    COPY web-portal-lite web-portal-lite
    COPY web-portal-lite-cli web-portal-lite-cli
    COPY web-portal-lite-core web-portal-lite-core

    RUN cargo build --target $BUILD_TARGET --release

    RUN cargo install --target $BUILD_TARGET --path ./web-portal-lite
    RUN cargo install --target $BUILD_TARGET --path ./web-portal-lite-cli

FROM alpine:3.16.0 as runtime

    WORKDIR /app

    COPY --from=builder /usr/local/cargo/bin/web_portal_lite .
    COPY --from=builder /usr/local/cargo/bin/web_portal_lite_cli .

    COPY Rocket.toml .
    COPY web-portal-lite/static /usr/src/app/web-portal-lite/static
    COPY templates /usr/src/app/web-portal-lite/templates

    EXPOSE 8000
    ENV ROCKET_CONFIG_PATH=/app/config.yml
    ENV ROCKET_ICONS_PATH=/app/icons
    ENV ROCKET_ADDRESS=0.0.0.0
    ENV ROCKET_PORT=8000
    ENV ROCKET_TEMPLATE_DIR=/usr/src/app/web-portal-lite/templates

    CMD ./web_portal_lite
