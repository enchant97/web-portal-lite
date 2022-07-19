# should be: x86_64-musl, aarch64-musl
ARG DOCKER_ARCH_TAG=x86_64-musl
# should be: x86_64-unknown-linux-musl, aarch64-unknown-linux-musl
ARG RUST_ARCH_TARGET=x86_64-unknown-linux-musl

FROM blackdex/rust-musl:$ARCH_TAG-stable as builder

    WORKDIR /usr/src/app

    COPY Cargo.toml Cargo.toml
    COPY web-portal-lite web-portal-lite
    COPY web-portal-lite-cli web-portal-lite-cli
    COPY web-portal-lite-core web-portal-lite-core

    RUN cargo build --release

FROM alpine:3.16.0

    WORKDIR /app

    COPY --from=builder /usr/src/app/target/$RUST_ARCH_TARGET/release/web_portal_lite .
    COPY --from=builder /usr/src/app/target/$RUST_ARCH_TARGET/release/web_portal_lite_cli .

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
